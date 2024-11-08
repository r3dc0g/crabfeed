use std::str::FromStr;

use crate::error::Error;
use crate::prelude::*;
use crate::AppResult;
use feed_rs::model;
use html_parser::{Dom, Node};
use log::debug;
use sqlx::query;
use sqlx::query_as;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use sqlx::SqliteConnection;

async fn setup_database(conn: &mut SqliteConnection) -> AppResult<()> {
    query!(
        "CREATE TABLE IF NOT EXISTS feed ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            title VARCHAR, \
            updated DATETIME, \
            description TEXT, \
            language VARCHAR, \
            published DATETIME \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS entry ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            feed_id INTEGER UNSIGNED NOT NULL, \
            title VARCHAR, \
            updated DATETIME, \
            content_id INTEGER, \
            summary TEXT, \
            source VARCHAR, \
            read BOOLEAN DEFAULT FALSE, \
            media_id INTEGER, \
            FOREIGN KEY(media_id) REFERENCES media(id) ON DELETE CASCADE, \
            FOREIGN KEY(feed_id) REFERENCES feed(id) ON DELETE CASCADE, \
            FOREIGN KEY(content_id) REFERENCES content(id) ON DELETE CASCADE \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS author ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            name VARCHAR NOT NULL, \
            uri VARCHAR, \
            email VARCHAR \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS link ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            href VARCHAR NOT NULL, \
            rel VARCHAR, \
            media_type VARCHAR, \
            href_lang VARCHAR, \
            title VARCHAR, \
            length BIGINT \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS content ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            body TEXT, \
            content_type VARCHAR, \
            length BIGINT, \
            src INTEGER, \
            FOREIGN KEY(src) REFERENCES link(id) ON DELETE CASCADE\
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS category ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            term VARCHAR NOT NULL, \
            scheme VARCHAR, \
            label VARCHAR \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS media ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            title VARCHAR, \
            thumbnail VARCHAR, \
            description VARCHAR \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS media_link ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            link_id INTEGER NOT NULL, \
            media_id INTEGER NOT NULL, \
            FOREIGN KEY(link_id) REFERENCES link(id), \
            FOREIGN KEY(media_id) REFERENCES media(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS feed_author ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            author_id INTEGER NOT NULL, \
            feed_id INTEGER NOT NULL, \
            FOREIGN KEY(author_id) REFERENCES author(id), \
            FOREIGN KEY(feed_id) REFERENCES feed(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS entry_author ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            author_id INTEGER NOT NULL, \
            entry_id INTEGER NOT NULL, \
            FOREIGN KEY(author_id) REFERENCES author(id), \
            FOREIGN KEY(entry_id) REFERENCES entry(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS feed_link ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            link_id INTEGER NOT NULL, \
            feed_id INTEGER NOT NULL, \
            FOREIGN KEY(link_id) REFERENCES link(id), \
            FOREIGN KEY(feed_id) REFERENCES feed(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS entry_link ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            link_id INTEGER NOT NULL, \
            entry_id INTEGER NOT NULL, \
            FOREIGN KEY(link_id) REFERENCES link(id), \
            FOREIGN KEY(entry_id) REFERENCES entry(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS feed_category ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            category_id INTEGER NOT NULL, \
            feed_id INTEGER NOT NULL, \
            FOREIGN KEY(category_id) REFERENCES category(id), \
            FOREIGN KEY(feed_id) REFERENCES feed(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    query!(
        "CREATE TABLE IF NOT EXISTS entry_category ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            category_id INTEGER NOT NULL, \
            entry_id INTEGER NOT NULL, \
            FOREIGN KEY(category_id) REFERENCES category(id), \
            FOREIGN KEY(entry_id) REFERENCES entry(id) \
        )",
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn connect(database_url: String) -> AppResult<SqliteConnection> {
    if cfg!(unix) {
        let mut conn = SqliteConnectOptions::from_str(&database_url)?
            .create_if_missing(true)
            .connect()
            .await?;
        setup_database(&mut conn).await?;
        return Ok(conn);
    }

    Err(Error::Static("Unsupported OS"))
}

pub async fn insert_feed(conn: &mut SqliteConnection, feed: model::Feed) -> AppResult<i64> {
    debug!("Starting Feed Insertion...");

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(feed.published)
        .build()?;

    match query_as!(
        Feed,
        r#"
        SELECT *
        FROM feed
        WHERE feed.title = ?
        "#,
        new_feed.title
    )
    .fetch_one(&mut *conn)
    .await
    {
        Ok(found_feed) => {
            debug!("Feed is already in DB");
            insert_entries(conn, feed.entries, found_feed.id).await?;

            return Ok(found_feed.id);
        }
        Err(_) => {
            debug!("Inserting New Feed");
            query!(
                r#"
                INSERT INTO feed (title, updated, description, language, published)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                new_feed.title,
                new_feed.updated,
                new_feed.description,
                new_feed.language,
                new_feed.published
            )
            .execute(&mut *conn)
            .await?;

            let ret_feed = query_as!(
                Feed,
                r#"
                SELECT *
                FROM feed
                WHERE feed.title = ?
                "#,
                new_feed.title
            )
            .fetch_one(&mut *conn)
            .await?;

            debug!("Populating feed data");
            insert_authors(conn, feed.authors, Some(ret_feed.id), None)
                .await
                .expect("Failed to insert feed authors");
            insert_entries(conn, feed.entries, ret_feed.id).await?;
            insert_links(conn, feed.links, Some(ret_feed.id), None)
                .await
                .expect("Failed to insert feed links");
            insert_categories(conn, feed.categories, Some(ret_feed.id), None)
                .await
                .expect("Failed to insert feed categories");
            return Ok(ret_feed.id);
        }
    }
}

pub async fn update_feed_title(
    conn: &mut SqliteConnection,
    feed_id: &i64,
    title: String,
) -> AppResult<()> {
    query!(
        r#"
        UPDATE feed
        SET title = $1
        WHERE feed.id = $2
        "#,
        title,
        feed_id
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn select_all_feeds(conn: &mut SqliteConnection) -> AppResult<Vec<Feed>> {
    let results = query_as!(
        Feed,
        r#"
        SELECT * FROM feed
        "#
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(results)
}

pub async fn select_feed(conn: &mut SqliteConnection, feed_id: &i64) -> AppResult<Feed> {
    debug!("Selecting Feed...");
    let result = query_as!(
        Feed,
        r#"
        SELECT *
        FROM feed
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_entry(conn: &mut SqliteConnection, entry_id: &i64) -> AppResult<Entry> {
    let result = query_as!(
        Entry,
        r#"
        SELECT *
        FROM entry
        WHERE entry.id = $1
        "#,
        entry_id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_content(conn: &mut SqliteConnection, content_id: &i64) -> AppResult<Content> {
    let result = query_as!(
        Content,
        r#"
        SELECT *
        FROM content
        WHERE content.id = $1
        "#,
        content_id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_feed_links(
    conn: &mut SqliteConnection,
    feed_id: &i64,
) -> AppResult<Vec<Link>> {
    let result = query_as!(
        Link,
        r#"
        SELECT link.id, link.href, link.rel, link.media_type, link.href_lang, link.title, link.length
        FROM link
        JOIN feed_link ON link.id = feed_link.link_id
        JOIN feed ON feed.id = feed_link.feed_id
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_feed_authors(
    conn: &mut SqliteConnection,
    feed_id: &i64,
) -> AppResult<Vec<Author>> {
    let result = query_as!(
        Author,
        r#"
        SELECT author.id, author.name, author.uri, author.email
        FROM author
        JOIN feed_author ON author.id = feed_author.author_id
        JOIN feed ON feed.id = feed_author.feed_id
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_feed_categories(
    conn: &mut SqliteConnection,
    feed_id: &i64,
) -> AppResult<Vec<Category>> {
    let result = query_as!(
        Category,
        r#"
        SELECT category.id, category.term, category.scheme, category.label
        FROM category
        JOIN feed_category ON category.id = feed_category.category_id
        JOIN feed ON feed.id = feed_category.feed_id
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_entry_links(
    conn: &mut SqliteConnection,
    entry_id: &i64,
) -> AppResult<Vec<Link>> {
    let result = query_as!(
        Link,
        r#"
        SELECT link.id, link.href, link.rel, link.media_type, link.href_lang, link.title, link.length
        FROM link
        JOIN entry_link ON link.id = entry_link.link_id
        JOIN entry ON entry.id = entry_link.entry_id
        WHERE entry.id = $1
        "#,
        entry_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_entry_authors(
    conn: &mut SqliteConnection,
    entry_id: &i64,
) -> AppResult<Vec<Author>> {
    let result = query_as!(
        Author,
        r#"
        SELECT author.id, author.name, author.uri, author.email
        FROM author
        JOIN entry_author ON author.id = entry_author.author_id
        JOIN entry ON entry.id = entry_author.entry_id
        WHERE entry.id = $1
        "#,
        entry_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_entry_categories(
    conn: &mut SqliteConnection,
    entry_id: &i64,
) -> AppResult<Vec<Category>> {
    let result = query_as!(
        Category,
        r#"
        SELECT category.id, category.term, category.scheme, category.label
        FROM category
        JOIN entry_category ON category.id = entry_category.category_id
        JOIN entry ON entry.id = entry_category.entry_id
        WHERE entry.id = $1
        "#,
        entry_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_media_links(
    conn: &mut SqliteConnection,
    media_id: &i64,
) -> AppResult<Vec<Link>> {
    let result = query_as!(
        Link,
        r#"
        SELECT link.id, link.href, link.rel, link.media_type, link.href_lang, link.title, link.length
        FROM link
        JOIN media_link ON link.id = media_link.link_id
        JOIN media ON media.id = media_link.media_id
        WHERE media.id = $1
        "#,
        media_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn select_all_entries(
    conn: &mut SqliteConnection,
    feed_id: &i64,
) -> AppResult<Vec<Entry>> {
    let entries = query_as!(
        Entry,
        r#"
        SELECT entry.id, entry.feed_id, entry.title, entry.updated, entry.content_id, entry.media_id, entry.summary, entry.source, entry.read
        FROM entry
        JOIN feed ON feed.id = entry.feed_id
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(entries)
}

pub async fn mark_entry_read(conn: &mut SqliteConnection, entry_id: &i64) -> AppResult<()> {
    query!(
        r#"
        UPDATE entry
        SET read = true
        WHERE entry.id = $1
        "#,
        entry_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn insert_entries(
    conn: &mut SqliteConnection,
    entries: Vec<model::Entry>,
    feed_id: i64,
) -> AppResult<()> {
    let mut builder = EntryBuilder::new();

    for entry in entries.iter().rev() {
        debug!("Starting Entry Insertion...");

        let content_id = insert_content(conn, entry.content.clone())
            .await
            .expect("Failed to insert entry content");

        let media_id = insert_media(conn, entry.media.first().cloned())
            .await
            .expect("Failed to insert entry media");

        let new_entry = builder
            .feed_id(feed_id)
            .title(entry.title.clone())
            .updated(entry.updated.clone())
            .content_id(content_id)
            .media_id(media_id)
            .summary(entry.summary.clone())
            .source(entry.source.clone())
            .build()?;

        let possible_entries = query_as!(
            Entry,
            r#"
            SELECT *
            FROM entry
            WHERE entry.title = ?
            "#,
            new_entry.title
        )
        .fetch_all(&mut *conn)
        .await?;

        if possible_entries.is_empty() {
            debug!("Inserting new entry...");
            query!(
                r#"
                INSERT INTO entry (feed_id, title, updated, content_id, summary, source, media_id)
                VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7
                )
                "#,
                new_entry.feed_id,
                new_entry.title,
                new_entry.updated,
                new_entry.content_id,
                new_entry.summary,
                new_entry.source,
                new_entry.media_id
            )
            .execute(&mut *conn)
            .await
            .expect("Failed to insert entry");

            debug!("Returning Entry ID...");
            let ret_entry = query_as!(
                Entry,
                r#"
                SELECT *
                FROM entry
                WHERE entry.feed_id = $1
                AND entry.title = $2
                "#,
                new_entry.feed_id,
                new_entry.title
            )
            .fetch_one(&mut *conn)
            .await?;

            debug!("Populating Entry data...");
            insert_authors(conn, entry.authors.clone(), None, Some(ret_entry.id))
                .await
                .expect("Failed to insert entry authors");
            insert_links(conn, entry.links.clone(), None, Some(ret_entry.id))
                .await
                .expect("Failed to insert entry links");
            insert_categories(conn, entry.categories.clone(), None, Some(ret_entry.id))
                .await
                .expect("Failed to insert entry categories");
        }
    }

    Ok(())
}

async fn insert_authors(
    conn: &mut SqliteConnection,
    authors: Vec<model::Person>,
    feed_id: Option<i64>,
    entry_id: Option<i64>,
) -> AppResult<()> {
    debug!("Starting Author Insertion...");
    let mut builder = AuthorBuilder::new();

    for person in authors {
        let new_author = builder
            .name(person.name)
            .uri(person.uri)
            .email(person.email)
            .build()?;

        let found_author = query_as!(
            Author,
            r#"
            SELECT *
            FROM author
            WHERE author.name = $1
            "#,
            new_author.name,
        )
        .fetch_one(&mut *conn)
        .await;

        if let Ok(ret_author) = found_author {
            debug!("Author is already in DB");
            let Some(f_id) = feed_id else {
                let Some(e_id) = entry_id else {
                    return Err(Error::Static("Orphaned Author"));
                };

                query!(
                    r#"
                    INSERT INTO entry_author (author_id, entry_id)
                    VALUES ($1, $2)
                    "#,
                    ret_author.id,
                    e_id
                )
                .execute(&mut *conn)
                .await?;

                continue;
            };

            query!(
                r#"
                INSERT INTO feed_author (author_id, feed_id)
                VALUES ($1, $2)
                "#,
                ret_author.id,
                f_id
            )
            .execute(&mut *conn)
            .await?;
        } else {
            debug!("Inserting new author...");
            query!(
                r#"
                INSERT INTO author (name, uri, email)
                VALUES ($1, $2, $3)
                "#,
                new_author.name,
                new_author.uri,
                new_author.email
            )
            .execute(&mut *conn)
            .await?;

            debug!("Returning Author ID...");
            let ret_author = query_as!(
                Author,
                r#"
                SELECT *
                FROM author
                WHERE author.name = ?
                "#,
                new_author.name
            )
            .fetch_one(&mut *conn)
            .await?;

            let Some(f_id) = feed_id else {
                let Some(e_id) = entry_id else {
                    return Err(Error::Static("Orphaned Author"));
                };

                query!(
                    r#"
                    INSERT INTO entry_author (author_id, entry_id)
                    VALUES ($1, $2)
                    "#,
                    ret_author.id,
                    e_id
                )
                .execute(&mut *conn)
                .await?;

                continue;
            };

            query!(
                r#"
                INSERT INTO feed_author (author_id, feed_id)
                VALUES ($1, $2)
                "#,
                ret_author.id,
                f_id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    Ok(())
}

pub async fn insert_links(
    conn: &mut SqliteConnection,
    links: Vec<model::Link>,
    feed_id: Option<i64>,
    entry_id: Option<i64>,
) -> AppResult<()> {
    debug!("Starting Link Insertion...");
    let mut builder = LinkBuilder::new();

    for link in links {
        let new_link = builder
            .href(link.href)
            .rel(link.rel)
            .media_type(link.media_type)
            .href_lang(link.href_lang)
            .title(link.title)
            .length(link.length)
            .build()?;

        debug!("Inserting new link...");
        query!(
            r#"
            INSERT INTO link (href, rel, media_type, href_lang, title, length)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            new_link.href,
            new_link.rel,
            new_link.media_type,
            new_link.href_lang,
            new_link.title,
            new_link.length
        )
        .execute(&mut *conn)
        .await?;

        debug!("Returning Link ID...");
        let ret_link = query_as!(
            Link,
            r#"
            SELECT *
            FROM link
            WHERE link.href = ?
            "#,
            new_link.href
        )
        .fetch_one(&mut *conn)
        .await?;

        let Some(f_id) = feed_id else {
            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Link"));
            };

            query!(
                r#"
                INSERT INTO entry_link (link_id, entry_id)
                VALUES ($1, $2)
                "#,
                ret_link.id,
                e_id
            )
            .execute(&mut *conn)
            .await?;

            continue;
        };

        query!(
            r#"
            INSERT INTO feed_link (link_id, feed_id)
            VALUES ($1, $2)
            "#,
            ret_link.id,
            f_id
        )
        .execute(&mut *conn)
        .await?;
    }

    if entry_id.is_some() {
        let e_id = entry_id.unwrap();

        let content_links = select_all_content_links(conn, e_id).await?;

        for link_string in content_links {
            insert_link(conn, link_string, None, Some(e_id)).await?;
        }
    }

    Ok(())
}

pub async fn insert_link(
    conn: &mut SqliteConnection,
    link: String,
    feed_id: Option<i64>,
    entry_id: Option<i64>,
) -> AppResult<()> {
    let mut builder = LinkBuilder::new();

    let new_link = builder
        .href(link.clone())
        .length(Some(link.len() as u64))
        .build()?;

    query!(
        r#"
        INSERT INTO link (href, rel, media_type, href_lang, title, length)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_link.href,
        new_link.rel,
        new_link.media_type,
        new_link.href_lang,
        new_link.title,
        new_link.length
    )
    .execute(&mut *conn)
    .await?;

    let ret_link = query_as!(
        Link,
        r#"
        SELECT *
        FROM link
        WHERE link.href = $1
        "#,
        new_link.href,
    )
    .fetch_one(&mut *conn)
    .await?;

    if let Some(f_id) = feed_id {
        query!(
            r#"
            insert into feed_link (link_id, feed_id)
            values ($1, $2)
            "#,
            ret_link.id,
            f_id
        )
        .execute(&mut *conn)
        .await?;
    } else {
        let Some(e_id) = entry_id else {
            return Err(Error::Static("Orphaned Link"));
        };

        query!(
            r#"
            insert into entry_link (link_id, entry_id)
            VALUES ($1, $2)
            "#,
            ret_link.id,
            e_id
        )
        .execute(&mut *conn)
        .await?;
    }

    Ok(())
}

async fn insert_categories(
    conn: &mut SqliteConnection,
    categories: Vec<model::Category>,
    feed_id: Option<i64>,
    entry_id: Option<i64>,
) -> AppResult<()> {
    let mut builder = CategoryBuilder::new();

    for category in categories {
        let new_category = builder
            .term(category.term)
            .scheme(category.scheme)
            .label(category.label)
            .build()?;

        query!(
            r#"
            INSERT INTO category (term, scheme, label)
            VALUES ($1, $2, $3)
            "#,
            new_category.term,
            new_category.scheme,
            new_category.label
        )
        .execute(&mut *conn)
        .await?;

        let ret_category = query_as!(
            Category,
            r#"
            SELECT *
            FROM category
            WHERE category.term = $1
            "#,
            new_category.term,
        )
        .fetch_one(&mut *conn)
        .await?;

        let Some(f_id) = feed_id else {
            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Category"));
            };

            query!(
                r#"
                INSERT INTO entry_category (category_id, entry_id)
                VALUES ($1, $2)
                "#,
                ret_category.id,
                e_id
            )
            .execute(&mut *conn)
            .await?;

            continue;
        };

        query!(
            r#"
            INSERT INTO feed_category (category_id, feed_id)
            VALUES ($1, $2)
            "#,
            ret_category.id,
            f_id
        )
        .execute(&mut *conn)
        .await?;
    }

    Ok(())
}

async fn insert_content(
    conn: &mut SqliteConnection,
    content_opt: Option<model::Content>,
) -> AppResult<Option<i64>> {
    let Some(content) = content_opt else {
        debug!("No Content for this entry");
        return Ok(None);
    };

    debug!("Starting Content Insertion...");

    let Some(link) = content.src else {
        let mut con_builder = ContentBuilder::new();

        let new_content = con_builder
            .body(content.body)
            .content_type(content.content_type)
            .length(content.length)
            .src(None)
            .build()?;

        query!(
            r#"
            INSERT INTO content (body, content_type, length, src)
            VALUES ($1, $2, $3, $4)
            "#,
            new_content.body,
            new_content.content_type,
            new_content.length,
            new_content.src
        )
        .execute(&mut *conn)
        .await?;

        let ret_content = query_as!(
            Content,
            r#"
            SELECT *
            FROM content
            WHERE content.body = ?
            AND content.content_type = ?
            AND content.src = ?
            "#,
            new_content.body,
            new_content.content_type,
            new_content.src
        )
        .fetch_one(&mut *conn)
        .await?;

        return Ok(Some(ret_content.id));
    };

    let mut con_builder = ContentBuilder::new();
    let mut link_builder = LinkBuilder::new();

    let new_link = link_builder
        .href(link.href)
        .rel(link.rel)
        .media_type(link.media_type)
        .href_lang(link.href_lang)
        .title(link.title)
        .length(link.length)
        .build()?;

    query!(
        r#"
        INSERT INTO link (href, rel, media_type, href_lang, title, length)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_link.href,
        new_link.rel,
        new_link.media_type,
        new_link.href_lang,
        new_link.title,
        new_link.length
    )
    .execute(&mut *conn)
    .await?;

    let ret_link = query_as!(
        Link,
        r#"
        SELECT *
        FROM link
        WHERE link.href = $1
        "#,
        new_link.href,
    )
    .fetch_one(&mut *conn)
    .await?;

    let new_content = con_builder
        .body(content.body)
        .content_type(content.content_type)
        .length(content.length)
        .src(Some(ret_link.id))
        .build()?;

    query!(
        r#"
        INSERT INTO content (body, content_type, length, src)
        VALUES (
            $1,
            $2,
            $3,
            $4
        )
        "#,
        new_content.body,
        new_content.content_type,
        new_content.length,
        new_content.src
    )
    .execute(&mut *conn)
    .await?;

    let ret_content = query_as!(
        Content,
        r#"
        SELECT *
        FROM content
        WHERE content.body = $1
        AND content.content_type = $2
        AND content.src = $3
        "#,
        new_content.body,
        new_content.content_type,
        new_content.src
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(Some(ret_content.id))
}

pub async fn insert_media(
    conn: &mut SqliteConnection,
    media: Option<model::MediaObject>,
) -> AppResult<Option<i64>> {
    let Some(media) = media else {
        debug!("No Media for this entry");
        return Ok(None);
    };

    debug!("Starting Media Insertion...");

    let mut media_builder = MediaBuilder::new();

    let new_media = media_builder
        .title(media.title)
        .thumbnail(match media.thumbnails.first() {
            Some(thumbnail) => Some(thumbnail.image.uri.clone()),
            None => None,
        })
        .description(media.description)
        .build()?;

    query!(
        r#"
        INSERT INTO media (title, thumbnail, description)
        VALUES ($1, $2, $3)
        "#,
        new_media.title,
        new_media.thumbnail,
        new_media.description
    )
    .execute(&mut *conn)
    .await
    .expect("Failed to insert media object");

    let ret_media = query_as!(
        Media,
        r#"
        SELECT *
        FROM MEDIA
        WHERE title = $1
        AND thumbnail = $2
        AND description = $3
        "#,
        new_media.title,
        new_media.thumbnail,
        new_media.description
    )
    .fetch_one(&mut *conn)
    .await
    .expect("Failed selecting inserted media object");

    for media_content in media.content.iter() {
        if let Some(link) = &media_content.url {
            let mut link_builder = LinkBuilder::new();

            let new_link = link_builder.href(link.to_string()).build()?;

            query!(
                r#"
                INSERT INTO link (href, rel, media_type, href_lang, title, length)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                new_link.href,
                new_link.rel,
                new_link.media_type,
                new_link.href_lang,
                new_link.title,
                new_link.length
            )
            .execute(&mut *conn)
            .await
            .expect("Failed inserting media link");

            let ret_link = query_as!(
                Link,
                r#"
                SELECT *
                FROM link
                WHERE link.href = $1
                "#,
                new_link.href,
            )
            .fetch_one(&mut *conn)
            .await?;

            query!(
                r#"
                INSERT INTO media_link (link_id, media_id)
                VALUES ($1, $2)
                "#,
                ret_link.id,
                ret_media.id
            )
            .execute(&mut *conn)
            .await
            .expect("Failed inserting media_link in linking table");
        }
    }

    return Ok(Some(ret_media.id));
}

pub async fn select_media(conn: &mut SqliteConnection, media_id: &i64) -> AppResult<Media> {
    let result = query_as!(
        Media,
        r#"
        SELECT *
        FROM media
        WHERE media.id = $1
        "#,
        media_id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(result)
}

pub async fn delete_feed(conn: &mut SqliteConnection, feed_id: i64) -> AppResult<()> {
    // Get all the entries for the feed
    // delete each entry's link, content and author
    // delete the entries, author, link, and category of the feed
    // delete the feed

    let entries = select_all_entries(conn, &feed_id).await?;

    for entry in entries {
        delete_entry(conn, entry.id).await?;
    }

    if let Ok(links) = select_all_feed_links(conn, &feed_id).await {
        for link in links {
            query!(
                r#"
                DELETE FROM feed_link
                WHERE feed_link.link_id = $1
                AND feed_link.feed_id = $2
                "#,
                link.id,
                feed_id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    if let Ok(authors) = select_all_feed_authors(conn, &feed_id).await {
        for author in authors {
            query!(
                r#"
                DELETE FROM feed_author
                WHERE feed_author.author_id = $1
                AND feed_author.feed_id = $2
                "#,
                author.id,
                feed_id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    if let Ok(categories) = select_all_feed_categories(conn, &feed_id).await {
        for category in categories {
            query!(
                r#"
                DELETE FROM feed_category
                WHERE feed_category.category_id = $1
                AND feed_category.feed_id = $2
                "#,
                category.id,
                feed_id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    query!(
        r#"
        DELETE FROM feed
        WHERE feed.id = $1
        "#,
        feed_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn delete_entry(conn: &mut SqliteConnection, entry_id: i64) -> AppResult<()> {
    // Get the entry
    // delete the entry's link, content and author
    // delete the entry

    let entry = select_entry(conn, &entry_id).await?;

    if let Ok(links) = select_all_entry_links(conn, &entry_id).await {
        for link in links {
            query!(
                r#"
                DELETE FROM entry_link
                WHERE entry_link.link_id = $1
                AND entry_link.entry_id = $2
                "#,
                link.id,
                entry.id
            )
            .execute(&mut *conn)
            .await?;

            query!(
                r#"
                DELETE FROM link
                WHERE link.id = $1
                "#,
                link.id
            )
            .execute(&mut *conn)
            .await?;
        }
    }
    if let Ok(authors) = select_all_entry_authors(conn, &entry.id).await {
        for author in authors {
            query!(
                r#"
                DELETE FROM entry_author
                WHERE entry_author.author_id = $1
                AND entry_author.entry_id = $2
                "#,
                author.id,
                entry.id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    if let Some(media_id) = entry.media_id {
        if let Ok(media_links) = select_all_media_links(conn, &media_id).await {
            for link in media_links {
                query!(
                    r#"
                    DELETE FROM media_link
                    WHERE media_link.link_id = $1
                    AND media_link.media_id = $2
                    "#,
                    link.id,
                    media_id
                )
                .execute(&mut *conn)
                .await?;
            }
        }
    };

    if let Ok(categories) = select_all_entry_categories(conn, &entry_id).await {
        for category in categories {
            query!(
                r#"
                DELETE FROM entry_category
                WHERE entry_category.category_id = $1
                AND entry_category.entry_id = $2
                "#,
                category.id,
                entry.id
            )
            .execute(&mut *conn)
            .await?;
        }
    }

    query!(
        r#"
        DELETE FROM entry
        WHERE entry.id = $1
        "#,
        entry.id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn select_all_content_links(
    conn: &mut SqliteConnection,
    entry_id: i64,
) -> AppResult<Vec<String>> {
    if let Ok(entry) = select_entry(conn, &entry_id).await {
        if let Some(content_id) = &entry.content_id {
            if let Ok(content) = select_content(conn, &content_id).await {
                if let Some(body) = &content.body {
                    return Ok(extract_links(body));
                }
            }
        } else {
            if let Some(summary) = &entry.summary {
                return Ok(extract_links(summary));
            }
        }
    }

    Ok(vec![])
}

fn extract_links(html: &str) -> Vec<String> {
    let mut links = vec![];

    if let Ok(dom) = Dom::parse(html) {
        let mut anchors = vec![];

        let mut adj_nodes = vec![];

        for node in dom.children {
            if let Node::Text(text) = node {
                if text.contains("http") {
                    links.push(text.to_string());
                }
            } else {
                adj_nodes.push(node);
            }
        }

        while !adj_nodes.is_empty() {
            let current_node = adj_nodes.remove(0);

            if let Node::Element(element) = current_node {
                if element.name == "a" {
                    anchors.push(element);
                } else {
                    for node in element.children {
                        if let Node::Text(text) = node {
                            if text.contains("http") {
                                links.push(text.to_string());
                            }
                        } else {
                            adj_nodes.push(node);
                        }
                    }
                }
            } else if let Node::Text(text) = current_node {
                if text.contains("http") {
                    links.push(text.to_string());
                }
            }
        }

        for anchor in anchors {
            if let Some(href_attr) = anchor.attributes.get("href") {
                if let Some(link) = href_attr {
                    links.push(link.to_string());
                }
            }
        }
    } else {
        return vec![];
    }

    links
}
