use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::prelude::{*, self};
use crate::schema::*;
use crate::error::Error;
use dotenvy::dotenv;
use feed_rs::model::{Feed, Entry, Person};
use std::env;

pub type Result<T> = core::result::Result<T, Error>;

pub fn connect() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL")?;

    let connection = SqliteConnection::establish(&database_url)?;

    Ok(connection)
}

pub fn insert_feed(conn: &mut SqliteConnection, feed: Feed) -> Result<()> {

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(feed.published)
        .build()?;

    let ret_feed: prelude::Feed = diesel::insert_into(feed::table)
        .values(&new_feed)
        .returning(prelude::Feed::as_returning())
        .get_result(conn)?;

    for entry in feed.entries {
        insert_entry(conn, entry, ret_feed.feed_id)?;
    }

    println!("{:#?}", feed.authors);

    insert_author(conn, feed.authors, Some(ret_feed.feed_id), None)?;

    Ok(())
}

pub fn insert_entry(conn: &mut SqliteConnection, entry: Entry, feed_id: i32) -> Result<()> {

    let mut builder = EntryBuilder::new();

    let new_entry = builder
        .feed_id(feed_id)
        .title(entry.title)
        .updated(entry.updated)
        .content_id(None)
        .summary(entry.summary)
        .source(entry.source)
        .build()?;

    let ret_entry: prelude::Entry = diesel::insert_into(entry::table)
        .values(&new_entry)
        .returning(prelude::Entry::as_returning())
        .get_result(conn)?;

    insert_author(conn, entry.authors, None, Some(ret_entry.entry_id))?;

    Ok(())
}

pub fn insert_author(conn: &mut SqliteConnection, authors: Vec<Person>, feed_id: Option<i32>, entry_id: Option<i32>) -> Result<()> {

    let mut builder = AuthorBuilder::new();

    for person in authors {

        let new_author = builder
            .name(person.name)
            .uri(person.uri)
            .email(person.email)
            .build()?;

        let ret_author: prelude::Author = diesel::insert_into(author::table)
            .values(&new_author)
            .returning(prelude::Author::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {

            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Author"));
            };

            let mut ea_builder = EntryAuthorBuilder::new();

            let entry_author = ea_builder
                .author_id(ret_author.author_id)
                .entry_id(e_id)
                .build()?;

            diesel::insert_into(entry_author::table)
                .values(&entry_author)
                .execute(conn)?;

            continue;
        };

        let mut fa_builder = FeedAuthorBuilder::new();

        let feed_author = fa_builder
            .author_id(ret_author.author_id)
            .feed_id(f_id)
            .build()?;

        diesel::insert_into(feed_author::table)
            .values(&feed_author)
            .execute(conn)?;
    }

    Ok(())
}
