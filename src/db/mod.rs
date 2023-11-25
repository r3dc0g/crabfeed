use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::prelude::*;
use dotenvy::dotenv;
use feed_rs::model::Feed;
use std::env;
use anyhow::Result;


pub fn connect() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url =  env::var("DATABASE_URL")?;

    let connection = SqliteConnection::establish(&database_url)?;

    Ok(connection)
}

pub fn insert_feed(conn: &mut SqliteConnection, feed: Feed) -> Result<()> {

    use crate::schema::feed;

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title(feed.title)
        .updated(feed.updated)
        .description(feed.description)
        .language(feed.language)
        .published(chrono::offset::Local::now().naive_utc())
        .build()?;

    diesel::insert_into(feed::table)
        .values(&new_feed)
        .execute(conn);

    Ok(())
}
