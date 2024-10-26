use crate::error::Error;
use crate::prelude::{EntryData, FeedData};
use crate::{time::TIME_STEP, AppResult};
use std::sync::mpsc;

use super::db::{
    connect, delete_feed, insert_feed, insert_link, mark_entry_read, select_all_entries,
    select_all_entry_links, select_all_feed_links, select_all_feeds, select_feed,
    update_feed_title,
};
use feed_rs::parser;
use reqwest;
use tokio::task::JoinHandle;

pub enum DataEvent {
    Complete,
    Updating(String),
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i64),
    Deleting(String),
    ReloadFeeds,
    ReloadedFeeds(Vec<FeedData>),
    ReloadEntries(i64),
    ReloadedEntries(Vec<EntryData>),
    ReadEntry(i64),
}

pub struct DataHandler {
    sender: mpsc::Sender<DataEvent>,
    receiver: mpsc::Receiver<DataEvent>,
    handler: JoinHandle<()>,
}

impl DataHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let (sender2, receiver2) = mpsc::channel();

        let handler = {
            let sender = sender2.clone();
            tokio::spawn(async move {
                while let Ok(event) = receiver.recv() {
                    if let Err(_) = DataHandler::handle_event(event, sender.clone()).await {
                        // TODO: Log error
                    }

                    if let Err(_) = sender.send(DataEvent::Complete) {
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

    pub fn dispatch(&self, event: DataEvent) -> AppResult<()> {
        self.sender.send(event)?;
        Ok(())
    }

    pub fn next(&self) -> AppResult<DataEvent> {
        let event = self.receiver.recv_timeout(TIME_STEP / 4)?;
        Ok(event)
    }

    // Handle Event
    pub async fn handle_event(event: DataEvent, sender: mpsc::Sender<DataEvent>) -> AppResult<()> {
        match event {
            DataEvent::UpdateFeeds => {
                DataHandler::update_feeds(sender).await?;
            }
            DataEvent::AddFeed(url) => {
                DataHandler::add_feed(url).await?;
            }
            DataEvent::DeleteFeed(id) => {
                DataHandler::delete_feed(sender, id).await?;
            }
            DataEvent::ReloadFeeds => {
                DataHandler::reload_feeds(sender).await?;
            }
            DataEvent::ReloadEntries(feed_id) => {
                DataHandler::reload_entries(sender, &feed_id).await?;
            }
            DataEvent::ReadEntry(entry_id) => {
                DataHandler::read_entry(&entry_id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(sender: mpsc::Sender<DataEvent>) -> AppResult<()> {
        let conn = &mut connect().await?;

        let feed_items = select_all_feeds(conn).await?;

        let mut new_feeds = vec![];

        for feed in feed_items.iter() {
            sender.send(DataEvent::Updating(format!(
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

    async fn delete_feed(sender: mpsc::Sender<DataEvent>, feed_id: i64) -> AppResult<()> {
        let conn = &mut connect().await?;

        let feed = select_feed(conn, &feed_id).await?;

        sender.send(DataEvent::Deleting(format!(
            "Deleting {}...",
            feed.title.unwrap_or("Untitled Feed".to_string())
        )))?;

        delete_feed(conn, feed_id).await?;

        Ok(())
    }

    async fn reload_feeds(sender: mpsc::Sender<DataEvent>) -> AppResult<()> {
        let conn = &mut connect().await?;

        let feeds = select_all_feeds(conn).await?;
        let mut feed_data = vec![];

        for feed in feeds {
            let mut data = FeedData::from(feed.clone());

            if let Some(link) = select_all_feed_links(conn, &feed.id).await?.first() {
                data.update_url(link.href.clone());
            }

            feed_data.push(data);
        }

        sender.send(DataEvent::ReloadedFeeds(feed_data))?;

        Ok(())
    }

    async fn reload_entries(sender: mpsc::Sender<DataEvent>, feed_id: &i64) -> AppResult<()> {
        let conn = &mut connect().await?;

        let entries = select_all_entries(conn, feed_id).await?;
        let mut entry_data = vec![];

        for entry in entries {
            let mut data = EntryData::from(entry.clone());

            // TODO: handle entry description

            let links = select_all_entry_links(conn, &entry.id).await?;
            data.update_links(links);
            entry_data.push(data);
        }

        sender.send(DataEvent::ReloadedEntries(entry_data))?;

        Ok(())
    }

    async fn read_entry(entry_id: &i64) -> AppResult<()> {
        let conn = &mut connect().await?;

        mark_entry_read(conn, entry_id);

        Ok(())
    }
}
