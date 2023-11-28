use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::prelude::*;
use dotenvy::dotenv;
use feed_rs::model::{Feed, Entry};
use std::env;
use anyhow::Result;


pub fn connect() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL")?;

    let connection = SqliteConnection::establish(&database_url)?;

    Ok(connection)
}

pub fn insert_feed(feed: Feed) -> Result<()> {

    let conn = &mut connect()?;

    use crate::schema::feed;

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(feed.published)
        .build()?;

    diesel::insert_into(feed::table)
        .values(&new_feed)
        .execute(conn)?;

    Ok(())
}

pub fn select_feed() -> Result<Vec<models::Feed>> {

    use crate::schema::feed::dsl::*;

    let connection = &mut connect()?;
    let result: Vec<models::Feed> = feed
        .select(models::Feed::as_select())
        .load(connection)?;

    Ok(result)
}

pub fn insert_entry(conn: &mut SqliteConnection, entry: Entry) -> Result<()> {
    todo!();
}
