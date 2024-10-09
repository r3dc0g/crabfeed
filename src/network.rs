use crate::error::Error;
use crate::{time::TIME_STEP, AppResult};
use std::sync::mpsc;

use crate::db::{self, delete_feed, find_feed_links, get_feeds, insert_feed, insert_link, update_feed_title};
use feed_rs::parser;
use reqwest;
use tokio::task::JoinHandle;

pub enum NetworkEvent {
    Complete,
    Updating(String),
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
            tokio::spawn(async move {
                while let Ok(event) = receiver.recv() {
                    if let Err(_) = NetworkHandler::handle_event(event, sender.clone()).await {
                        // TODO: Log error
                    }

                    if let Err(_) = sender.send(NetworkEvent::Complete) {
                        // TODO: Log error
                    }
                }
            })
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
    pub async fn handle_event(
        event: NetworkEvent,
        sender: mpsc::Sender<NetworkEvent>,
    ) -> AppResult<()> {
        match event {
            NetworkEvent::UpdateFeeds => {
                NetworkHandler::update_feeds(sender).await?;
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
    async fn update_feeds(sender: mpsc::Sender<NetworkEvent>) -> AppResult<()> {
        let feed_items = get_feeds()?;

        let mut new_feeds = vec![];

        for feed in feed_items.iter() {
            sender.send(NetworkEvent::Updating(format!(
                "Updating {}...",
                feed.title.clone().unwrap_or("Untitled Feed".to_string())
            )))?;

            let links = find_feed_links(feed.id)?;

            if links.is_empty() {
                return Err(Error::Static("No links found for feed"));
            }

            for link in links.iter() {
                if let Ok(res) = reqwest::get(link.href.clone()).await {
                    if let Ok(content) = res.text().await {
                        let new_feed = parser::parse(content.as_bytes());

                        if let Ok(neofeed) = new_feed {
                            if let Some(new_title) = &neofeed.title {
                                if let Some(old_title) = &feed.title {
                                    if new_title.content != *old_title {
                                        update_feed_title(&feed.id, new_title.content.clone())?;
                                    }
                                }
                            }
                            new_feeds.push(neofeed);
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
        let content = reqwest::get(feed_url.as_str()).await?.text().await?;

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
