use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::prelude::*;
use crate::schema::*;
use crate::error::Error;
use dotenvy::dotenv;
use feed_rs::model;
use std::env;

pub type Result<T> = core::result::Result<T, Error>;

pub fn connect() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL")?;

    let connection = SqliteConnection::establish(&database_url)?;

    Ok(connection)
}

pub fn insert_feed(
        conn: &mut SqliteConnection,
        feed: model::Feed
    ) -> Result<()> {

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(feed.published)
        .build()?;

    let found_feed = feed::table
        .filter(feed::title.eq(&new_feed.title))
        .select(Feed::as_select())
        .get_result(conn);

    if found_feed.is_ok() {
        return Err(Error::Static("Feed already exists"));
    }

    let ret_feed: Feed = diesel::insert_into(feed::table)
        .values(&new_feed)
        .returning(Feed::as_returning())
        .get_result(conn)?;

    insert_authors(conn, feed.authors, Some(ret_feed.id), None)?;
    insert_links(conn, feed.links, Some(ret_feed.id), None)?;
    insert_entries(conn, feed.entries, ret_feed.id)?;

    Ok(())
}

pub fn get_feeds() -> Result<Vec<Feed>> {

    use crate::schema::feed::dsl::*;

    let conn = &mut connect()?;

    let results = feed
        .load::<Feed>(conn)?;

    Ok(results)

}

pub fn select_feed(feed_id: &i32) -> Result<Feed> {

    let conn = &mut connect()?;

    let result = feed::table
        .filter(feed::id.eq(feed_id))
        .select(Feed::as_select())
        .get_result(conn)?;

    Ok(result)

}

pub fn find_feed_link(feed_id: i32) -> Result<Link> {

    let conn = &mut connect()?;

    let result = feed::table
        .inner_join(feed_link::table.inner_join(link::table))
        .filter(feed::id.eq(feed_id))
        .select(Link::as_select())
        .get_result(conn)?;

    Ok(result)
}

pub fn get_entries(curr_feed: &Feed) -> Result<Vec<Entry>> {

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

fn insert_entries(
        conn: &mut SqliteConnection,
        entries: Vec<model::Entry>,
        feed_id: i32
    ) -> Result<()> {

    let mut builder = EntryBuilder::new();

    for entry in entries {

        let content_id = insert_content(conn, entry.content)?;

        let new_entry = builder
            .feed_id(feed_id)
            .title(entry.title)
            .updated(entry.updated)
            .content_id(content_id)
            .summary(entry.summary)
            .source(entry.source)
            .build()?;

        let ret_entry: Entry = diesel::insert_into(entry::table)
            .values(&new_entry)
            .returning(Entry::as_returning())
            .get_result(conn)?;

        insert_authors(conn, entry.authors, None, Some(ret_entry.id))?;
        insert_links(conn, entry.links, None, Some(ret_entry.id))?;

    }

    Ok(())
}

fn insert_authors(
        conn: &mut SqliteConnection,
        authors: Vec<model::Person>,
        feed_id: Option<i32>,
        entry_id: Option<i32>
    ) -> Result<()> {

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

            let entry_author = ea_builder
                .author_id(ret_author.id)
                .entry_id(e_id)
                .build()?;

            diesel::insert_into(entry_author::table)
                .values(&entry_author)
                .execute(conn)?;

            continue;
        };

        let mut fa_builder = FeedAuthorBuilder::new();

        let feed_author = fa_builder
            .author_id(ret_author.id)
            .feed_id(f_id)
            .build()?;

        diesel::insert_into(feed_author::table)
            .values(&feed_author)
            .execute(conn)?;
    }

    Ok(())
}

fn insert_links(
        conn: &mut SqliteConnection,
        links: Vec<model::Link>,
        feed_id: Option<i32>,
        entry_id: Option<i32>,
    ) -> Result<()> {

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

            let entry_link = el_builder
                .link_id(ret_link.id)
                .entry_id(e_id)
                .build()?;

            diesel::insert_into(entry_link::table)
                .values(&entry_link)
                .execute(conn)?;

            continue;
        };

        let mut fl_builder = FeedLinkBuilder::new();

        let feed_link = fl_builder
            .link_id(ret_link.id)
            .feed_id(f_id)
            .build()?;

        diesel::insert_into(feed_link::table)
            .values(&feed_link)
            .execute(conn)?;

    }

    Ok(())
}

fn insert_categories(
        conn: &mut SqliteConnection,
        categories: Vec<model::Category>,
        feed_id: Option<i32>,
        entry_id: Option<i32>
    ) -> Result<()> {

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
    ) -> Result<Option<i32>> {

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
