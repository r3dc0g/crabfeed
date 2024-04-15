use crate::app::App;
use crate::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::control::get_feed;
use crate::db;

pub type Result<T> = core::result::Result<T, Error>;

pub enum IOEvent {
    FetchFeeds,

}

pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Self { app }
    }

    // Handle IOEvent
    pub async fn handle_io_event(&self, event: IOEvent) -> Result<()> {
        match event {
            IOEvent::FetchFeeds => self.update_feeds().await?,
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(&self) -> Result<()> {
        let mut app = self.app.lock().await;

        // Grab the feeds from the app state
        let feeds = &app.feeds;
        let mut new_feeds = vec![];

        // Fetch the feed model for each feed
        for feed in feeds {
            let links = &feed.links;
            for link in links {
                let link = &link.href;
                let feed = match get_feed(link).await {
                    Ok(feed) => feed,

                    // dont add feed if we cant fetch it
                    Err(_) => continue,
                };
                new_feeds.push(feed);
            }
        }

        // Update the app state
        app.feeds = new_feeds;

        //Update the database
        for feed in &app.feeds {
            let connection = &mut db::connect()?;

            // Skip if feed already exists
            if let Err(_) = db::insert_feed(connection, feed.clone()) {
                continue;
            }
        }

        Ok(())
    }
}

