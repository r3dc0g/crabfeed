use anyhow::Result;
use feed_rs::{parser, model::Feed};
use std::fs::read_to_string;
use html2text::{from_read};

async fn get_feed(source: String) -> Result<Feed> {
    let content = reqwest::get(source)
        .await?
        .text()
        .await?;

    let feed = parser::parse(content.as_bytes())?;

    Ok(feed)
}
