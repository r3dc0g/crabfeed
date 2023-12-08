mod control;
mod db;
mod schema;
mod prelude;
mod error;
mod ui;

use anyhow::Result;
// use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {

    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    #[tokio::test]
    async fn test_feed_insertion() -> Result<()> {
        use crate::control::get_feed;
        use crate::db::*;

        let test_feed = get_feed("https://www.midwesternmarx.com/1/feed").await?;

        let conn = &mut connect()?;

        insert_feed(conn, test_feed)?;

        Ok(())
    }
}
