use crate::app::App;
use crate::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::control::get_feed;
use crate::db::{self, find_feed_link, get_feeds, insert_feed};

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

        let connection = &mut db::connect()?;

        // Grab the feeds from the app state
        let feeds = &app.feeds;

        if feeds.is_empty() {

            // Fetch feeds from the database
            app.feeds = get_feeds()?;
            app.is_loading = false;
            return Ok(());
        }

        let mut new_feeds = vec![];

        // Fetch the feed model for each feed
        for feed in feeds {
            let link = find_feed_link(feed.id)?;

            let Ok(feed) = get_feed(link.href).await else {
                continue;
            };

            new_feeds.push(feed);

        }

        //Update the database
        for feed in new_feeds {
            insert_feed(connection, feed)?
        }
        // Update the app state
        app.feeds = get_feeds()?;

        app.is_loading = false;

        Ok(())
    }
}

