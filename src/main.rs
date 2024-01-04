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
    use std::fs::read_to_string;

    #[tokio::test]
    async fn test_feed_insertion() -> Result<()> {
        use crate::control::get_feed;
        use crate::db::*;

        let mut lines = Vec::new();

        for line in read_to_string("urls")?.lines() {
            lines.push(line.to_string());
        }

        let conn = &mut connect()?;

        for line in lines {
            match get_feed(line).await {
                Ok(test_feed) => {
                    insert_feed(conn, test_feed)?;
                },
                Err(e) => {
                    println!("{:?}", e);
                    ()
                }
            }

        }

        Ok(())
    }
}
