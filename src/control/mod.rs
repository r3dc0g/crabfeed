use anyhow::Result;
use feed_rs::{parser, model::Feed};

pub async fn get_feed(source: impl Into<String>) -> Result<Feed> {
    let content = reqwest::get(source.into())
        .await?
        .text()
        .await?;

    let feed = parser::parse(content.as_bytes())?;

    Ok(feed)
}
