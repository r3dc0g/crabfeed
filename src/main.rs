mod control;
mod db;
mod schema;
mod prelude;
mod models;
mod error;

use anyhow::Result;
// use crate::prelude::*;
use control::get_feed;
use db::*;

#[tokio::main]
async fn main() -> Result<()> {

    Ok(())
}
