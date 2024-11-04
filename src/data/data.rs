use crate::error::Error;
use crate::prelude::{EntryData, FeedData};
use crate::AppResult;

use super::db::{
    connect, delete_feed, insert_feed, insert_link, mark_entry_read, select_all_entries,
    select_all_entry_links, select_all_feed_links, select_all_feeds, select_feed,
    update_feed_title,
};
use feed_rs::parser;
use log::debug;
use reqwest;

#[derive(Debug)]
pub enum DataEvent {
    Complete,
    Error(Box<Error>),
    Updating(String),
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i64),
    Deleting(String),
    ReloadFeeds,
    ReloadedFeeds(Vec<FeedData>),
    ReloadEntries(Vec<i64>),
    ReloadedEntries(Vec<Vec<EntryData>>),
    ReadEntry(i64),
}

pub struct DataHandler {
    sender: std::sync::mpsc::Sender<DataEvent>,
    receiver: tokio::sync::mpsc::Receiver<DataEvent>,
    handler: tokio::task::JoinHandle<()>,
}

impl DataHandler {
    pub fn new(database_url: String) -> Self {
        debug!("Creating Data channels");
        let (sync_sender, sync_receiver) = std::sync::mpsc::channel();
        let (async_sender, async_receiver) = tokio::sync::mpsc::channel(32);

        debug!("Spawning Data Handler thread");
        let handler = tokio::spawn({
            async move {
                let moved_sender = async_sender.clone();
                loop {
                    if let Ok(event) = sync_receiver.recv() {
                        debug!("Handling {:?}", event);
                        DataHandler::handle_event(
                            database_url.clone(),
                            event,
                            moved_sender.clone(),
                        )
                        .await
                        .expect("Failed to handle Event");
                    }
                }
            }
        });

        Self {
            sender: sync_sender,
            receiver: async_receiver,
            handler,
        }
    }

    pub fn dispatch(&self, event: DataEvent) -> AppResult<()> {
        Ok(self.sender.send(event).expect("Failed to send event"))
    }

    pub fn next(&mut self) -> AppResult<DataEvent> {
        Ok(self.receiver.try_recv()?)
    }

    pub fn check(&self) -> bool {
        !self.handler.is_finished()
    }

    pub fn abort(&self) {
        self.handler.abort();
    }

    // Handle Event
    pub async fn handle_event(
        database_url: String,
        event: DataEvent,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
    ) -> AppResult<()> {
        debug!("Handling Data Event...");
        match event {
            DataEvent::UpdateFeeds => {
                DataHandler::update_feeds(database_url, sender.clone()).await?;
            }
            DataEvent::AddFeed(url) => {
                DataHandler::add_feed(database_url, url, sender.clone()).await?;
            }
            DataEvent::DeleteFeed(id) => {
                DataHandler::delete_feed(database_url, sender.clone(), id).await?;
            }
            DataEvent::ReloadFeeds => {
                DataHandler::reload_feeds(database_url, sender.clone()).await?;
            }
            DataEvent::ReloadEntries(feed_ids) => {
                DataHandler::reload_entries(database_url, sender.clone(), &feed_ids).await?;
            }
            DataEvent::ReadEntry(entry_id) => {
                DataHandler::read_entry(database_url, &entry_id, sender.clone()).await?;
            }
            _ => {}
        }

        Ok(())
    }

    // Fetch feeds and update the app state
    async fn update_feeds(
        database_url: String,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
    ) -> AppResult<()> {
        debug!("Updating Feeds...");

        let conn = &mut connect(database_url).await?;

        let feed_items = select_all_feeds(conn).await?;

        let mut new_feeds = vec![];

        for feed in feed_items.iter() {
            sender
                .send(DataEvent::Updating(format!(
                    "Updating {}...",
                    feed.title.clone().unwrap_or("Untitled Feed".to_string())
                )))
                .await
                .expect("Failed to send DataEvent::Updating");

            let links = select_all_feed_links(conn, &feed.id).await?;

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

        sender
            .send(DataEvent::Complete)
            .await
            .expect("Failed to send DataEvent::Complete");

        Ok(())
    }

    async fn add_feed(
        database_url: String,
        feed_url: String,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
    ) -> AppResult<()> {
        debug!("Adding {feed_url}...");
        let content = reqwest::get(feed_url.as_str()).await?.text().await?;

        let feed = parser::parse(content.as_bytes());

        if let Ok(feed) = feed {
            let conn = &mut connect(database_url).await?;
            let feed_id = insert_feed(conn, feed).await?;

            insert_link(conn, feed_url, Some(feed_id), None).await?;
        }

        sender
            .send(DataEvent::Complete)
            .await
            .expect("Failed to send DataEvent::Complete");

        Ok(())
    }

    async fn delete_feed(
        database_url: String,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
        feed_id: i64,
    ) -> AppResult<()> {
        debug!("Deleting Feed...");
        let conn = &mut connect(database_url).await?;

        let feed = select_feed(conn, &feed_id).await?;

        sender
            .send(DataEvent::Deleting(format!(
                "Deleting {}...",
                feed.title.unwrap_or("Untitled Feed".to_string())
            )))
            .await
            .expect("Failed to send Deleting event");

        delete_feed(conn, feed_id).await?;

        sender
            .send(DataEvent::Complete)
            .await
            .expect("Failed to send DataEvent::Complete");

        Ok(())
    }

    async fn reload_feeds(
        database_url: String,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
    ) -> AppResult<()> {
        debug!("Reloading Feeds...");
        let conn = &mut connect(database_url)
            .await
            .expect("Failed to connect to Database");

        let feeds = select_all_feeds(conn)
            .await
            .expect("Failed to select all Feeds");
        let mut feed_data = vec![];

        for feed in feeds {
            let mut data = FeedData::from(feed.clone());

            if let Some(link) = select_all_feed_links(conn, &feed.id)
                .await
                .expect("Failed to connect to Database")
                .first()
            {
                data.update_url(link.href.clone());
            }

            feed_data.push(data);
        }

        debug!("Sending Feeds out of Data Handler");
        sender
            .send(DataEvent::ReloadedFeeds(feed_data))
            .await
            .expect("Failed to send ReloadedFeeds");

        // A Complete is not sent so that reloading the entries continues

        Ok(())
    }

    async fn reload_entries(
        database_url: String,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
        feed_ids: &Vec<i64>,
    ) -> AppResult<()> {
        let conn = &mut connect(database_url).await?;

        let mut entry_groups = vec![];

        for id in feed_ids {
            let entries = select_all_entries(conn, id).await?;
            let mut entry_data = vec![];

            for entry in entries {
                let mut data = EntryData::from(entry.clone());

                // TODO: handle entry description

                let links = select_all_entry_links(conn, &entry.id).await?;
                data.update_links(links);
                entry_data.push(data);
            }

            entry_groups.push(entry_data);
        }

        debug!("Sending Entries out of Data Handler");
        sender
            .send(DataEvent::ReloadedEntries(entry_groups))
            .await
            .expect("Filed to send ReloadedEntries");

        debug!("Complete ReloadedEntries");
        sender
            .send(DataEvent::Complete)
            .await
            .expect("Failed to send DataEvent::Complete");

        Ok(())
    }

    async fn read_entry(
        database_url: String,
        entry_id: &i64,
        sender: tokio::sync::mpsc::Sender<DataEvent>,
    ) -> AppResult<()> {
        let conn = &mut connect(database_url).await?;

        mark_entry_read(conn, entry_id).await?;

        sender
            .send(DataEvent::Complete)
            .await
            .expect("Failed to send DataEvent::Complete");

        Ok(())
    }
}
