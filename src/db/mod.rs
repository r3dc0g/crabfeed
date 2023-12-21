#![allow(dead_code)]

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::prelude::{*, self};
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

pub fn insert_feed(conn: &mut SqliteConnection, feed: model::Feed) -> Result<()> {

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

    println!("{:#?}", feed.authors);
    insert_authors(conn, feed.authors, Some(ret_feed.feed_id), None)?;
    insert_links(conn, feed.links, Some(ret_feed.feed_id), None)?;
    insert_entries(conn, feed.entries, ret_feed.feed_id)?;

    Ok(())
}

pub fn insert_entries(conn: &mut SqliteConnection, entries: Vec<model::Entry>, feed_id: i32) -> Result<()> {

    let mut builder = EntryBuilder::new();

    for entry in entries {

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

        println!("{:#?}", entry.authors);
        insert_authors(conn, entry.authors, None, Some(ret_entry.entry_id))?;
        insert_links(conn, entry.links, None, Some(ret_entry.entry_id))?;

    }


    Ok(())
}

pub fn insert_authors(conn: &mut SqliteConnection, authors: Vec<model::Person>, feed_id: Option<i32>, entry_id: Option<i32>) -> Result<()> {

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

pub fn insert_links(conn: &mut SqliteConnection, links: Vec<model::Link>, feed_id: Option<i32>, entry_id: Option<i32>) -> Result<()> {

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

        let ret_link: prelude::Link = diesel::insert_into(link::table)
            .values(&new_link)
            .returning(prelude::Link::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {

            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Link"));
            };

            let mut el_builder = EntryLinkBuilder::new();

            let entry_link = el_builder
                .link_id(ret_link.link_id)
                .entry_id(e_id)
                .build()?;

            diesel::insert_into(entry_link::table)
                .values(&entry_link)
                .execute(conn)?;

            continue;
        };

        let mut fl_builder = FeedLinkBuilder::new();

        let feed_link = fl_builder
            .link_id(ret_link.link_id)
            .feed_id(f_id)
            .build()?;

        diesel::insert_into(feed_link::table)
            .values(&feed_link)
            .execute(conn)?;

    }

    Ok(())
}

pub fn insert_categories(conn: &mut SqliteConnection, categories: Vec<model::Category>, feed_id: Option<i32>, entry_id: Option<i32>) -> Result<()> {

    let mut builder = CategoryBuilder::new();

    for category in categories {

        let new_category = builder
            .term(category.term)
            .scheme(category.scheme)
            .label(category.label)
            .build()?;

        let ret_category: prelude::Category = diesel::insert_into(category::table)
            .values(&new_category)
            .returning(prelude::Category::as_returning())
            .get_result(conn)?;

        let Some(f_id) = feed_id else {

            let Some(e_id) = entry_id else {
                return Err(Error::Static("Orphaned Category"));
            };

            let mut ec_builder = EntryCategoryBuilder::new();

            let entry_category = ec_builder
                .category_id(ret_category.category_id)
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
