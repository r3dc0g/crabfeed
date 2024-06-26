use crate::app::App;
use crate::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::control::get_feed;
use crate::db::{self, find_feed_links, get_feeds, insert_feed};

pub type Result<T> = core::result::Result<T, Error>;

pub enum IOEvent {
    FetchFeeds,
    AddFeed(String),
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
            IOEvent::FetchFeeds => {
                self.update_feeds().await?;
            }
            IOEvent::AddFeed(url) => {
                self.add_feed(url).await?;
            }
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(&self) -> Result<()> {
        let mut app = self.app.lock().await;

        // Grab the feeds from the app state
        let feed_items = &app.feed_items;

        if feed_items.is_empty() {
            // Fetch feeds from the database
            app.set_feed_items(get_feeds()?);
            app.is_loading = false;
            return Ok(());
        }

        let mut new_feeds = vec![];

        // Fetch the feed model for each feed
        for feed in get_feeds()?.iter() {
            let links = find_feed_links(feed.id)?;

            for link in links {
                if let Ok(feed) = get_feed(link.href).await {
                    new_feeds.push(feed);
                };
            }

        }

        let connection = &mut db::connect()?;

        //Update the database
        for feed in new_feeds {
            insert_feed(connection, feed)?
        }

        // Update the app state
        app.update_feed_items();

        app.is_loading = false;

        Ok(())
    }

    async fn add_feed(&self, feed_url: String) -> Result<()> {

        let new_feed = get_feed(feed_url).await?;
        let connection = &mut db::connect()?;
        insert_feed(connection, new_feed)?;

        let mut app = self.app.lock().await;
        app.update_feed_items();
        app.is_loading = false;

        Ok(())
    }
}

