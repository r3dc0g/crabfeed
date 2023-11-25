mod db;
mod schema;
mod prelude;
mod models;
mod error;

use anyhow::Result;
use crate::prelude::*;

fn main() -> Result<()> {

    let mut builder = FeedBuilder::new();

    let new_feed = builder
        .title("test")
        .updated("Now")
        .description("first test")
        .language("English")
        .published("Not yet")
        .rating("Good")
        .rights("Open source")
        .build()?;

    println!("{new_feed:#?}");

    Ok(())
}
