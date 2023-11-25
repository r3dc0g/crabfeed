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

pub fn insert_feed(feed: Feed) -> Result<()> {

    Ok(())
}
