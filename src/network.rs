use crate::app::{App, RouteId, ActiveBlock};
use crate::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use feed_rs::parser;
use reqwest;
use crate::db::{self, find_feed_links, get_feeds, insert_feed, insert_link, delete_feed};

pub type Result<T> = core::result::Result<T, Error>;

pub enum IOEvent {
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i32),
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
            IOEvent::UpdateFeeds => {
                self.update_feeds().await?;
            }
            IOEvent::AddFeed(url) => {
                self.add_feed(url).await?;
            }
            IOEvent::DeleteFeed(id) => {
                self.delete_feed(id).await?;
            }
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(&self) -> Result<()> {
        // Grab the feeds from the app state
        let feed_items = get_feeds()?;

        if feed_items.is_empty() {
            let mut app = self.app.lock().await;
            app.is_loading = false;
            return Ok(());
        }

        let mut new_feeds = vec![];

        // Fetch the feed model for each feed
        for (i, feed) in feed_items.iter().enumerate() {

            let feed_name = feed.title.clone().unwrap_or("Unnamed Feed".to_string());

            {
                let mut app = self.app.lock().await;
                app.loading_msg = format!("({}/{}) Updating {}...", i + 1, feed_items.len(), feed_name);
            }

            let links = find_feed_links(feed.id)?;

            if links.is_empty() {
                return Err(Error::Static("No links found for feed"));
            }

            for link in links.iter() {
                if  let Ok(res) = reqwest::get(link.href.clone()).await {
                    if let Ok(content) = res.text().await {
                        let new_feed = parser::parse(content.as_bytes());

                        if let Ok(feed) = new_feed {
                            new_feeds.push(feed);
                        };
                    }
                }
            }


        }

        let connection = &mut db::connect()?;

        //Update the database
        for feed in new_feeds {
            insert_feed(connection, feed)?;
        }

        let mut app = self.app.lock().await;
        // Update the app state
        app.update_feed_items();
        app.set_current_route(RouteId::Home, ActiveBlock::Feeds);
        app.is_loading = false;
        Ok(())
    }

    async fn add_feed(&self, feed_url: String) -> Result<()> {

        let content = reqwest::get(feed_url.as_str())
            .await?
            .text()
            .await?;

        let feed = parser::parse(content.as_bytes());

        if let Ok(feed) = feed {
            let connection = &mut db::connect()?;
            let feed_id = insert_feed(connection, feed)?;

            insert_link(connection, feed_url, Some(feed_id), None)?;
        }

        let mut app = self.app.lock().await;
        app.update_feed_items();
        app.is_loading = false;

        Ok(())
    }

    async fn delete_feed(&self, feed_id: i32) -> Result<()> {

        delete_feed(feed_id)?;

        let mut app = self.app.lock().await;
        let feed_index = app.feed_list_state.selected().unwrap_or(0);
        app.feed_items.remove(feed_index);
        app.is_loading = false;
        app.update_feed_items();
        Ok(())
    }

}

