use crate::{time::TIME_STEP, AppResult};
use crate::error::Error;
use std::sync::mpsc;

use feed_rs::parser;
use reqwest;
use tokio::task::JoinHandle;
use crate::db::{self, find_feed_links, get_feeds, insert_feed, insert_link, delete_feed};

pub enum NetworkEvent {
    Complete,
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i32),
}

pub struct NetworkHandler {
    sender: mpsc::Sender<NetworkEvent>,
    receiver: mpsc::Receiver<NetworkEvent>,
    handler: JoinHandle<()>,
}

impl NetworkHandler {
    pub fn new() -> Self {

        let (sender, receiver) = mpsc::channel();
        let (sender2, receiver2) = mpsc::channel();

        let handler = {
            let sender = sender2.clone();
            tokio::spawn(
                async move {
                    while let Ok(event) = receiver.recv() {
                        if let Err(_) = NetworkHandler::handle_event(event).await {
                            // TODO: Log error
                        }

                        if let Err(_) = sender.send(NetworkEvent::Complete) {
                            // TODO: Log error
                        }
                    }
                }
            )
        };

        Self {
            sender,
            receiver: receiver2,
            handler,
        }
    }

    pub fn dispatch(&self, event: NetworkEvent) -> AppResult<()> {
        self.sender.send(event)?;
        Ok(())
    }

    pub fn next(&self) -> AppResult<NetworkEvent> {
        let event = self.receiver.recv_timeout(TIME_STEP / 4)?;
        Ok(event)
    }

    // Handle Event
    pub async fn handle_event(event: NetworkEvent) -> AppResult<()> {
        match event {
            NetworkEvent::UpdateFeeds => {
                NetworkHandler::update_feeds().await?;
            }
            NetworkEvent::AddFeed(url) => {
                NetworkHandler::add_feed(url).await?;
            }
            NetworkEvent::DeleteFeed(id) => {
                NetworkHandler::delete_feed(id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds() -> AppResult<()> {
        let feed_items = get_feeds()?;

        let mut new_feeds = vec![];

        for feed in feed_items.iter() {

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

        Ok(())
    }

    async fn add_feed(feed_url: String) -> AppResult<()> {

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

        Ok(())
    }

    async fn delete_feed(feed_id: i32) -> AppResult<()> {

        delete_feed(feed_id)?;

        Ok(())
    }

}
