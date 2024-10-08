use crate::error::Error;
use crate::prelude::*;
use crate::schema::*;
use crate::AppResult;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use feed_rs::model;
use html_parser::{Dom, Node};
use std::env;
use std::fs;
use std::fs::create_dir_all;

fn setup_database(conn: &mut SqliteConnection) -> AppResult<()> {
    sql_query(
        "CREATE TABLE feed ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        title VARCHAR, \
        updated DATETIME, \
        description TEXT, \
        language VARCHAR, \
        published DATETIME \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE entry ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        feed_id INTEGER UNSIGNED NOT NULL, \
        title VARCHAR, \
        updated DATETIME, \
        content_id INTEGER, \
        summary TEXT, \
        source VARCHAR, \
        read BOOLEAN DEFAULT FALSE, \
        media_id INTEGER, \
        FOREIGN KEY(media_id) REFERENCES media(media_id), \
        FOREIGN KEY(feed_id) REFERENCES feed(feed_id), \
        FOREIGN KEY(content_id) REFERENCES content(content_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE author ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        name VARCHAR NOT NULL, \
        uri VARCHAR, \
        email VARCHAR \
     )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE link ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        href VARCHAR NOT NULL, \
        rel VARCHAR, \
        media_type VARCHAR, \
        href_lang VARCHAR, \
        title VARCHAR, \
        length BIGINT \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE content ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        body TEXT, \
        content_type VARCHAR, \
        length BIGINT, \
        src INTEGER, \
        FOREIGN KEY(src) REFERENCES link(link_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE category ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        term VARCHAR NOT NULL, \
        scheme VARCHAR, \
        label VARCHAR \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE media ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        title VARCHAR, \
        thumbnail VARCHAR, \
        description VARCHAR \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE media_link ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        link_id INTEGER NOT NULL, \
        media_id INTEGER NOT NULL, \
        FOREIGN KEY(link_id) REFERENCES link(link_id), \
        FOREIGN KEY(media_id) REFERENCES media(media_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE feed_author ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        author_id INTEGER NOT NULL, \
        feed_id INTEGER NOT NULL, \
        FOREIGN KEY(author_id) REFERENCES author(author_id), \
        FOREIGN KEY(feed_id) REFERENCES feed(feed_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE entry_author ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        author_id INTEGER NOT NULL, \
        entry_id INTEGER NOT NULL, \
        FOREIGN KEY(author_id) REFERENCES author(author_id), \
        FOREIGN KEY(entry_id) REFERENCES entry(entry_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE feed_link ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        link_id INTEGER NOT NULL, \
        feed_id INTEGER NOT NULL, \
        FOREIGN KEY(link_id) REFERENCES link(link_id), \
        FOREIGN KEY(feed_id) REFERENCES feed(feed_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE entry_link ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        link_id INTEGER NOT NULL, \
        entry_id INTEGER NOT NULL, \
        FOREIGN KEY(link_id) REFERENCES link(link_id), \
        FOREIGN KEY(entry_id) REFERENCES entry(entry_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE feed_category ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        category_id INTEGER NOT NULL, \
        feed_id INTEGER NOT NULL, \
        FOREIGN KEY(category_id) REFERENCES category(category_id), \
        FOREIGN KEY(feed_id) REFERENCES feed(feed_id) \
    )",
    )
    .execute(conn)?;

    sql_query(
        "CREATE TABLE entry_category ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
        category_id INTEGER NOT NULL, \
        entry_id INTEGER NOT NULL, \
        FOREIGN KEY(category_id) REFERENCES category(category_id), \
        FOREIGN KEY(entry_id) REFERENCES entry(entry_id) \
    )",
    )
    .execute(conn)?;

    Ok(())
}

pub fn connect() -> AppResult<SqliteConnection> {
    if cfg!(unix) {
        let mut first_creation = false;
        let home = env::var("HOME").unwrap();
        let database_folder = format!("{}/.local/share/crabfeed/", home);
        let database_url = format!("{}/crabfeed.db", database_folder);
        create_dir_all(database_folder)?;

        if fs::metadata(&database_url).is_err() {
            first_creation = true;
        }

        let mut connection = SqliteConnection::establish(&database_url)?;

        if first_creation {
            setup_database(&mut connection)?;
        }

        return Ok(connection);
    }

    Err(Error::Static("Unsupported OS"))
}

pub fn insert_feed(conn: &mut SqliteConnection, feed: model::Feed) -> AppResult<i32> {
    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(feed.published)
        .build()?;

    let possible_feed = feed::table
        .filter(feed::title.eq(&new_feed.title))
        .select(Feed::as_select())
        .get_result(conn);

    if let Ok(found_feed) = possible_feed {
        insert_entries(conn, feed.entries, found_feed.id)?;

        return Ok(found_feed.id);
    } else {
        let ret_feed: Feed = diesel::insert_into(feed::table)
            .values(&new_feed)
            .returning(Feed::as_returning())
            .get_result(conn)?;

        insert_authors(conn, feed.authors, Some(ret_feed.id), None)?;
        insert_entries(conn, feed.entries, ret_feed.id)?;
        insert_links(conn, feed.links, Some(ret_feed.id), None)?;
        insert_categories(conn, feed.categories, Some(ret_feed.id), None)?;
        return Ok(ret_feed.id);
    }
}

pub fn update_feed_title(feed_id: &i32, title: String) -> AppResult<()> {
    let conn = &mut connect()?;

    diesel::update(feed::table.filter(feed::id.eq(feed_id)))
        .set(feed::title.eq(title))
        .execute(conn)?;

    Ok(())
}

pub fn get_feeds() -> AppResult<Vec<Feed>> {
    use crate::schema::feed::dsl::*;

    let conn = &mut connect()?;

    let results = feed.load::<Feed>(conn)?;

    Ok(results)
}

pub fn select_feed(feed_id: &i32) -> AppResult<Feed> {
    let conn = &mut connect()?;

    let result = feed::table
        .filter(feed::id.eq(feed_id))
        .select(Feed::as_select())
        .get_result(conn)?;

    Ok(result)
}

pub fn select_entry(entry_id: &i32) -> AppResult<Entry> {
    let conn = &mut connect()?;

    let result = entry::table
        .filter(entry::id.eq(entry_id))
        .select(Entry::as_select())
        .get_result(conn)?;

    Ok(result)
}

pub fn select_content(content_id: &i32) -> AppResult<Content> {
    let conn = &mut connect()?;

    let result = content::table
        .filter(content::id.eq(content_id))
        .select(Content::as_select())
        .get_result(conn)?;

    Ok(result)
}

pub fn find_feed_links(feed_id: i32) -> AppResult<Vec<Link>> {
    let conn = &mut connect()?;

    let result = feed::table
        .inner_join(feed_link::table.inner_join(link::table))
        .filter(feed::id.eq(feed_id))
        .select(Link::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_feed_authors(feed_id: i32) -> AppResult<Vec<Author>> {
    let conn = &mut connect()?;

    let result = feed::table
        .inner_join(feed_author::table.inner_join(author::table))
        .filter(feed::id.eq(feed_id))
        .select(Author::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_feed_categories(feed_id: i32) -> AppResult<Vec<Category>> {
    let conn = &mut connect()?;

    let result = feed::table
        .inner_join(feed_category::table.inner_join(category::table))
        .filter(feed::id.eq(feed_id))
        .select(Category::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_entry_links(entry_id: i32) -> AppResult<Vec<Link>> {
    let conn = &mut connect()?;

    let result = entry::table
        .inner_join(entry_link::table.inner_join(link::table))
        .filter(entry::id.eq(entry_id))
        .select(Link::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_entry_authors(entry_id: i32) -> AppResult<Vec<Author>> {
    let conn = &mut connect()?;

    let result = entry::table
        .inner_join(entry_author::table.inner_join(author::table))
        .filter(entry::id.eq(entry_id))
        .select(Author::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_entry_categories(entry_id: i32) -> AppResult<Vec<Category>> {
    let conn = &mut connect()?;

    let result = entry::table
        .inner_join(entry_category::table.inner_join(category::table))
        .filter(entry::id.eq(entry_id))
        .select(Category::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn find_media_links(media_id: i32) -> AppResult<Vec<Link>> {
    let conn = &mut connect()?;

    let result = media::table
        .inner_join(media_link::table.inner_join(link::table))
        .filter(media::id.eq(media_id))
        .select(Link::as_select())
        .get_results(conn)?;

    Ok(result)
}

pub fn get_entries(curr_feed: &Feed) -> AppResult<Vec<Entry>> {
    let conn = &mut connect()?;

    let feed_id = feed::table
        .filter(feed::title.eq(&curr_feed.title))
        .select(Feed::as_select())
        .get_result(conn)?;

    let entries = Entry::belonging_to(&feed_id)
        .select(Entry::as_select())
        .load(conn)?;

    Ok(entries)
}

pub fn select_entries(feed_id: i32) -> AppResult<Vec<Entry>> {
    let conn = &mut connect()?;

    let entries = entry::table
        .filter(entry::feed_id.eq(feed_id))
        .select(Entry::as_select())
        .load(conn)?;

    Ok(entries)
}

pub fn mark_entry_read(entry_id: i32) -> AppResult<()> {
    let conn = &mut connect()?;

    diesel::update(entry::table.filter(entry::id.eq(entry_id)))
        .set(entry::read.eq(true))
        .execute(conn)?;

    Ok(())
}

fn insert_entries(
    conn: &mut SqliteConnection,
    entries: Vec<model::Entry>,
    feed_id: i32,
) -> AppResult<()> {
    let mut builder = EntryBuilder::new();

    for entry in entries.iter().rev() {
        let possible_entries: Vec<Entry> = entry::table
            .filter(entry::title.eq(entry.title.clone().unwrap().content))
            .select(Entry::as_select())
            .get_results(conn)?;

        if possible_entries.len() > 0 {
            continue;
        }

        let content_id = insert_content(conn, entry.content.clone())?;

        let media_id = insert_media(conn, entry.media.first().cloned())?;

        let new_entry = builder
            .feed_id(feed_id)
            .title(entry.title.clone())
            .updated(entry.updated.clone())
            .content_id(content_id)
            .media_id(media_id)
            .summary(entry.summary.clone())
            .source(entry.source.clone())
            .build()?;

        let ret_entry: Entry = diesel::insert_into(entry::table)
            .values(&new_entry)
            .returning(Entry::as_returning())
            .get_result(conn)?;

        insert_authors(conn, entry.authors.clone(), None, Some(ret_entry.id))?;
        insert_links(conn, entry.links.clone(), None, Some(ret_entry.id))?;
        insert_categories(conn, entry.categories.clone(), None, Some(ret_entry.id))?;
    }

    Ok(())
}

fn insert_authors(
    conn: &mut SqliteConnection,
    authors: Vec<model::Person>,
    feed_id: Option<i32>,
    entry_id: Option<i32>,
) -> AppResult<()> {
    let mut builder = AuthorBuilder::new();

    for person in authors {
        let new_author = builder
            .name(person.name)
            .uri(person.uri)
            .email(person.email)
            .build()?;

        let ret_author: Author = diesel::insert_into(author::table)
            .values(&new_author)
            .returning(Author::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {
            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Author"));
            };

            let mut ea_builder = EntryAuthorBuilder::new();

            let entry_author = ea_builder.author_id(ret_author.id).entry_id(e_id).build()?;

            diesel::insert_into(entry_author::table)
                .values(&entry_author)
                .execute(conn)?;

            continue;
        };

        let mut fa_builder = FeedAuthorBuilder::new();

        let feed_author = fa_builder.author_id(ret_author.id).feed_id(f_id).build()?;

        diesel::insert_into(feed_author::table)
            .values(&feed_author)
            .execute(conn)?;
    }

    Ok(())
}

pub fn insert_links(
    conn: &mut SqliteConnection,
    links: Vec<model::Link>,
    feed_id: Option<i32>,
    entry_id: Option<i32>,
) -> AppResult<()> {
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

        let ret_link: Link = diesel::insert_into(link::table)
            .values(&new_link)
            .returning(Link::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {
            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Link"));
            };

            let mut el_builder = EntryLinkBuilder::new();

            let entry_link = el_builder.link_id(ret_link.id).entry_id(e_id).build()?;

            diesel::insert_into(entry_link::table)
                .values(&entry_link)
                .execute(conn)?;

            continue;
        };

        let mut fl_builder = FeedLinkBuilder::new();

        let feed_link = fl_builder.link_id(ret_link.id).feed_id(f_id).build()?;

        diesel::insert_into(feed_link::table)
            .values(&feed_link)
            .execute(conn)?;
    }

    if entry_id.is_some() {
        let e_id = entry_id.unwrap();

        let content_links = get_content_links(e_id);

        for link_string in content_links {
            insert_link(conn, link_string, None, Some(e_id))?;
        }
    }

    Ok(())
}

pub fn insert_link(
    conn: &mut SqliteConnection,
    link: String,
    feed_id: Option<i32>,
    entry_id: Option<i32>,
) -> AppResult<()> {
    let mut builder = LinkBuilder::new();

    let new_link = builder
        .href(link.clone())
        .length(Some(link.len() as u64))
        .build()?;

    let ret_link: Link = diesel::insert_into(link::table)
        .values(&new_link)
        .returning(Link::as_returning())
        .get_result(conn)?;

    if let Some(f_id) = feed_id {
        let mut fl_builder = FeedLinkBuilder::new();

        let feed_link = fl_builder.link_id(ret_link.id).feed_id(f_id).build()?;

        diesel::insert_into(feed_link::table)
            .values(&feed_link)
            .execute(conn)?;
    } else {
        let Some(e_id) = entry_id else {
            return Err(Error::Static("Orphaned Link"));
        };

        let mut el_builder = EntryLinkBuilder::new();

        let entry_link = el_builder.link_id(ret_link.id).entry_id(e_id).build()?;

        diesel::insert_into(entry_link::table)
            .values(&entry_link)
            .execute(conn)?;
    }

    Ok(())
}

fn insert_categories(
    conn: &mut SqliteConnection,
    categories: Vec<model::Category>,
    feed_id: Option<i32>,
    entry_id: Option<i32>,
) -> AppResult<()> {
    let mut builder = CategoryBuilder::new();

    for category in categories {
        let new_category = builder
            .term(category.term)
            .scheme(category.scheme)
            .label(category.label)
            .build()?;

        let ret_category: Category = diesel::insert_into(category::table)
            .values(&new_category)
            .returning(Category::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {
            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Category"));
            };

            let mut ec_builder = EntryCategoryBuilder::new();

            let entry_category = ec_builder
                .category_id(ret_category.id)
                .entry_id(e_id)
                .build()?;

            diesel::insert_into(entry_category::table)
                .values(&entry_category)
                .execute(conn)?;

            continue;
        };

        let mut fc_builder = FeedCategoryBuilder::new();

        let feed_category = fc_builder
            .category_id(ret_category.id)
            .feed_id(f_id)
            .build()?;

        diesel::insert_into(feed_category::table)
            .values(&feed_category)
            .execute(conn)?;
    }

    Ok(())
}

fn insert_content(
    conn: &mut SqliteConnection,
    content_opt: Option<model::Content>,
) -> AppResult<Option<i32>> {
    let Some(content) = content_opt else {
        return Ok(None);
    };

    let Some(link) = content.src else {
        let mut con_builder = ContentBuilder::new();

        let new_content = con_builder
            .body(content.body)
            .content_type(content.content_type)
            .length(content.length)
            .src(None)
            .build()?;

        let ret_content = diesel::insert_into(content::table)
            .values(&new_content)
            .returning(Content::as_returning())
            .get_result(conn)?;

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

    let ret_link: Link = diesel::insert_into(link::table)
        .values(&new_link)
        .returning(Link::as_returning())
        .get_result(conn)?;

    let new_content = con_builder
        .body(content.body)
        .content_type(content.content_type)
        .length(content.length)
        .src(Some(ret_link.id))
        .build()?;

    let ret_content = diesel::insert_into(content::table)
        .values(&new_content)
        .returning(Content::as_returning())
        .get_result(conn)?;

    Ok(Some(ret_content.id))
}

pub fn insert_media(
    conn: &mut SqliteConnection,
    media: Option<model::MediaObject>,
) -> AppResult<Option<i32>> {
    let Some(media) = media else {
        return Ok(None);
    };

    let mut media_builder = MediaBuilder::new();

    let Some(thumbnail) = media.thumbnails.first() else {
        let new_media = media_builder
            .title(media.title)
            .thumbnail(None)
            .description(media.description)
            .build()?;

        let ret_media = diesel::insert_into(media::table)
            .values(&new_media)
            .returning(Media::as_returning())
            .get_result(conn)?;

        return Ok(Some(ret_media.id));
    };

    let new_media = media_builder
        .title(media.title)
        .thumbnail(Some(thumbnail.image.uri.clone()))
        .description(media.description)
        .build()?;

    let ret_media = diesel::insert_into(media::table)
        .values(&new_media)
        .returning(Media::as_returning())
        .get_result(conn)?;

    for media_content in media.content.iter() {
        if let Some(link) = &media_content.url {
            let mut link_builder = LinkBuilder::new();

            let new_link = link_builder.href(link.to_string()).build()?;

            let ret_link = diesel::insert_into(link::table)
                .values(&new_link)
                .returning(Link::as_returning())
                .get_result(conn)?;

            let mut media_link_builder = MediaLinkBuilder::new();

            let new_media_link = media_link_builder
                .link_id(ret_link.id)
                .media_id(ret_media.id)
                .build()?;

            diesel::insert_into(media_link::table)
                .values(&new_media_link)
                .execute(conn)?;
        }
    }

    return Ok(Some(ret_media.id));
}

pub fn select_media(media_id: &i32) -> AppResult<Media> {
    let conn = &mut connect()?;

    let result = media::table
        .filter(media::id.eq(media_id))
        .select(Media::as_select())
        .get_result(conn)?;

    Ok(result)
}

pub fn delete_feed(feed_id: i32) -> AppResult<()> {
    // Get all the entries for the feed
    // delete each entry's link, content and author
    // delete the entries, author, link, and category of the feed
    // delete the feed

    let conn = &mut connect()?;
    let feed = select_feed(&feed_id)?;
    let entries = get_entries(&feed)?;

    for entry in entries {
        delete_entry(entry.id)?;
    }

    if let Ok(links) = find_feed_links(feed_id) {
        for link in links {
            diesel::delete(feed_link::table.filter(feed_link::feed_id.eq(feed_id)))
                .execute(conn)?;

            diesel::delete(link::table.filter(link::id.eq(link.id))).execute(conn)?;
        }
    }

    if let Ok(authors) = find_feed_authors(feed_id) {
        for author in authors {
            diesel::delete(feed_author::table.filter(feed_author::feed_id.eq(feed_id)))
                .execute(conn)?;

            diesel::delete(author::table.filter(author::id.eq(author.id))).execute(conn)?;
        }
    }

    if let Ok(categories) = find_feed_categories(feed_id) {
        for category in categories {
            diesel::delete(feed_category::table.filter(feed_category::feed_id.eq(feed_id)))
                .execute(conn)?;

            diesel::delete(category::table.filter(category::id.eq(category.id))).execute(conn)?;
        }
    }

    diesel::delete(feed::table.filter(feed::id.eq(feed_id))).execute(conn)?;

    Ok(())
}

pub fn delete_entry(entry_id: i32) -> AppResult<()> {
    // Get the entry
    // delete the entry's link, content and author
    // delete the entry

    let conn = &mut connect()?;
    let entry = select_entry(&entry_id)?;

    if let Ok(links) = find_entry_links(entry.id) {
        for link in links {
            diesel::delete(entry_link::table.filter(entry_link::entry_id.eq(entry.id)))
                .execute(conn)?;

            diesel::delete(link::table.filter(link::id.eq(link.id))).execute(conn)?;
        }
    }
    if let Ok(authors) = find_entry_authors(entry.id) {
        for author in authors {
            diesel::delete(entry_author::table.filter(entry_author::entry_id.eq(entry.id)))
                .execute(conn)?;

            diesel::delete(author::table.filter(author::id.eq(author.id))).execute(conn)?;
        }
    }

    if let Some(content_id) = entry.content_id {
        diesel::delete(content::table.filter(content::id.eq(content_id))).execute(conn)?;
    };

    if let Some(media_id) = entry.media_id {
        if let Ok(media_links) = find_media_links(media_id) {
            for media_link in media_links {
                diesel::delete(media_link::table.filter(media_link::id.eq(media_link.id)))
                    .execute(conn)?;
            }
        }

        diesel::delete(media_link::table.filter(media_link::media_id.eq(media_id)))
            .execute(conn)?;

        diesel::delete(media::table.filter(media::id.eq(media_id))).execute(conn)?;
    };

    if let Ok(categories) = find_entry_categories(entry.id) {
        for category in categories {
            diesel::delete(entry_category::table.filter(entry_category::entry_id.eq(entry.id)))
                .execute(conn)?;

            diesel::delete(category::table.filter(category::id.eq(category.id))).execute(conn)?;
        }
    }

    diesel::delete(entry::table.filter(entry::id.eq(entry.id))).execute(conn)?;

    Ok(())
}

fn get_content_links(entry_id: i32) -> Vec<String> {
    if let Ok(entry) = select_entry(&entry_id) {
        if let Some(content_id) = &entry.content_id {
            if let Ok(content) = select_content(content_id) {
                if let Some(body) = &content.body {
                    return extract_links(body);
                } else {
                    return vec![];
                }
            } else {
                return vec![];
            }
        } else {
            if let Some(summary) = &entry.summary {
                return extract_links(summary);
            } else {
                return vec![];
            }
        }
    } else {
        return vec![];
    }
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

#[test]
fn delete_entry_test() {
    assert!(delete_entry(25).is_ok());
}
