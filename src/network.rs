use crate::error::Error;
use crate::{time::TIME_STEP, AppResult};
use std::sync::mpsc;

use crate::db::{
    connect, delete_feed, insert_feed, insert_link, select_all_feed_links, select_all_feeds,
    select_feed, update_feed_title,
};
use feed_rs::parser;
use reqwest;
use tokio::task::JoinHandle;

pub enum NetworkEvent {
    Complete,
    Updating(String),
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i64),
    Deleting(String),
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
                NetworkHandler::delete_feed(sender, id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(sender: mpsc::Sender<NetworkEvent>) -> AppResult<()> {
        let conn = &mut connect().await?;

        let feed_items = select_all_feeds(conn).await?;

        let mut new_feeds = vec![];

        for feed in feed_items.iter() {
            sender.send(NetworkEvent::Updating(format!(
                "Updating {}...",
                feed.title.clone().unwrap_or("Untitled Feed".to_string())
            )))?;

            let links = select_all_feed_links(conn, &feed.id).await?;

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
                                        update_feed_title(
                                            conn,
                                            &feed.id,
                                            new_title.content.clone(),
                                        )
                                        .await?;
                                    }
                                }
                            }
                            new_feeds.push(neofeed);
                        };
                    }
                }
            }
        }

        //Update the database
        for feed in new_feeds {
            insert_feed(conn, feed).await?;
        }

        Ok(())
    }

    async fn add_feed(feed_url: String) -> AppResult<()> {
        let content = reqwest::get(feed_url.as_str()).await?.text().await?;

        let feed = parser::parse(content.as_bytes());

        if let Ok(feed) = feed {
            let conn = &mut connect().await?;
            let feed_id = insert_feed(conn, feed).await?;

            insert_link(conn, feed_url, Some(feed_id), None).await?;
        }

        Ok(())
    }

    async fn delete_feed(sender: mpsc::Sender<NetworkEvent>, feed_id: i64) -> AppResult<()> {
        let conn = &mut connect().await?;

        let feed = select_feed(conn, &feed_id).await?;

        sender.send(NetworkEvent::Deleting(format!(
            "Deleting {}...",
            feed.title.unwrap_or("Untitled Feed".to_string())
        )))?;

        delete_feed(conn, feed_id).await?;

        Ok(())
    }
}
