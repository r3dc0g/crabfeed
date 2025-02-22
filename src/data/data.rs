use std::process::exit;

use crate::app::AppEvent;
use crate::error::Error;
use crate::prelude::{Entry, EntryData, FeedData};
use crate::AppResult;

use super::db::{
    self, connect, insert_feed, insert_link, mark_entry_read, select_all_entries,
    select_all_entry_links, select_all_feed_links, select_all_feeds, select_content, select_feed,
    select_media, update_feed_title,
};
use feed_rs::parser;
use log::debug;
use reqwest;
use sqlx::SqliteConnection;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct Cache {
    pub feeds: Vec<FeedData>,
    pub entries: Vec<Vec<EntryData>>,
}

#[derive(Debug)]
pub enum DataEvent {
    Error(Box<Error>),
    Updating(String),
    UpdateFeeds,
    AddFeed(String),
    DeleteFeed(i64),
    Refresh,
    ReadEntry(i64),
    Abort,
}

pub struct DataHandler {
    sender: std::sync::mpsc::Sender<DataEvent>,
    receiver: tokio::sync::mpsc::Receiver<AppEvent>,
    handler: tokio::task::AbortHandle,
}

impl DataHandler {
    pub fn new(database_url: String) -> Self {
        debug!("Creating Data channels");
        let (sync_sender, sync_receiver) = std::sync::mpsc::channel();
        let (async_sender, async_receiver) = tokio::sync::mpsc::channel(32);

        debug!("Spawning Data Handler thread");
        let handler = tokio::spawn(async move {
            let moved_sender = async_sender.clone();
            loop {
                if let Ok(event) = sync_receiver.recv() {
                    debug!("Handling {:?}", event);
                    match event {
                        DataEvent::Abort => {
                            exit(0);
                        }
                        _ => {
                            handle_event(database_url.clone(), event, moved_sender.clone())
                                .await
                                .expect("Failed to handle Event");
                        }
                    }
                }
            }
        })
        .abort_handle();

        Self {
            sender: sync_sender,
            receiver: async_receiver,
            handler,
        }
    }

    pub fn dispatch(&self, event: DataEvent) -> AppResult<()> {
        Ok(self.sender.send(event).expect("Failed to send event"))
    }

    pub fn next(&mut self) -> AppResult<AppEvent> {
        Ok(self.receiver.try_recv()?)
    }

    pub fn check(&self) -> bool {
        !self.handler.is_finished()
    }
}

// Handle Event
pub async fn handle_event(
    database_url: String,
    event: DataEvent,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    debug!("Handling Data Event...");
    match event {
        DataEvent::UpdateFeeds => {
            update_feeds(database_url, sender.clone()).await?;
        }
        DataEvent::AddFeed(url) => {
            add_feed(database_url, url, sender.clone()).await?;
        }
        DataEvent::DeleteFeed(id) => {
            delete_feed(database_url, sender.clone(), id).await?;
        }
        DataEvent::Refresh => {
            refresh(database_url, sender.clone()).await?;
        }
        DataEvent::ReadEntry(entry_id) => {
            read_entry(database_url, &entry_id, sender.clone()).await?;
        }
        _ => {}
    }

    Ok(())
}

// Fetch feeds and update the app state
async fn update_feeds(
    database_url: String,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    debug!("Updating Feeds...");

    let conn = &mut connect(database_url).await?;

    let feed_items = select_all_feeds(conn).await?;

    let mut new_feeds = vec![];

    for feed in feed_items.iter() {
        sender
            .send(AppEvent::DisplayMsg(format!(
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
                                    update_feed_title(conn, &feed.id, new_title.content.clone())
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
        .send(AppEvent::Complete)
        .await
        .expect("Failed to send DataEvent::Complete");

    Ok(())
}

async fn add_feed(
    database_url: String,
    feed_url: String,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    debug!("Adding {feed_url}...");

    let Ok(response) = reqwest::get(feed_url.as_str()).await else {
        sender
            .send(AppEvent::DisplayMsg("Could not find feed".to_string()))
            .await
            .expect("Failed to send AppEvent::DisplayMsg");

        sleep(Duration::from_secs(1)).await;

        sender
            .send(AppEvent::Complete)
            .await
            .expect("Failed to send AppEvent::Complete");

        return Ok(());
    };

    let content = response.text().await.expect("Failed to get feed data");

    let feed = parser::parse(content.as_bytes());

    if let Ok(feed) = feed {
        if feed.title.is_none() {
            sender
                .send(AppEvent::DisplayMsg("Adding Untitled Feed...".to_string()))
                .await
                .expect("Failed to send AppEvent::DisplayMsg");
        } else {
            sender
                .send(AppEvent::DisplayMsg(format!(
                    "Adding {}...",
                    feed.title.clone().unwrap().content
                )))
                .await
                .expect("Failed to send AppEvent::DisplayMsg");
        }

        let conn = &mut connect(database_url)
            .await
            .expect("Failed to connect to Database");
        let feed_id = insert_feed(conn, feed).await?;

        insert_link(conn, feed_url, Some(feed_id), None)
            .await
            .expect("Failed to insert link");
    }

    sender
        .send(AppEvent::Complete)
        .await
        .expect("Failed to send AppEvent::Complete");

    Ok(())
}

async fn delete_feed(
    database_url: String,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
    feed_id: i64,
) -> AppResult<()> {
    debug!("Deleting Feed...");
    let conn = &mut connect(database_url).await?;

    let feed = select_feed(conn, &feed_id).await?;

    sender
        .send(AppEvent::DisplayMsg(format!(
            "Deleting {}...",
            feed.title.unwrap_or("Untitled Feed".to_string())
        )))
        .await
        .expect("Failed to send Deleting event");

    db::delete_feed(conn, feed_id).await?;

    sender
        .send(AppEvent::Complete)
        .await
        .expect("Failed to send AppEvent::Complete");

    Ok(())
}

async fn refresh(
    database_url: String,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    debug!("Refreshing data...");

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

    let feed_ids: Vec<i64> = feed_data.iter().map(|f| f.id).collect();
    let mut entry_groups = vec![];

    for id in feed_ids {
        let entries = select_all_entries(conn, &id).await?;
        let mut entry_data = vec![];

        for entry in entries {
            let mut data = EntryData::from(entry.clone());

            data.description = process_entry_description(conn, &entry).await?;

            let links = select_all_entry_links(conn, &entry.id).await?;
            data.update_links(links);
            entry_data.push(data);
        }

        entry_groups.push(entry_data);
    }

    sender
        .send(AppEvent::FeshData(Cache {
            feeds: feed_data,
            entries: entry_groups,
        }))
        .await
        .expect("Failed to send AppEvent::FeshData");

    sender
        .send(AppEvent::Complete)
        .await
        .expect("Failed to send AppEvent::Complete");

    Ok(())
}

async fn read_entry(
    database_url: String,
    entry_id: &i64,
    sender: tokio::sync::mpsc::Sender<AppEvent>,
) -> AppResult<()> {
    let conn = &mut connect(database_url).await?;

    mark_entry_read(conn, entry_id).await?;

    sender
        .send(AppEvent::Complete)
        .await
        .expect("Failed to send AppEvent::Complete");

    Ok(())
}

async fn process_entry_description(
    conn: &mut SqliteConnection,
    entry: &Entry,
) -> AppResult<String> {
    let mut content = String::new();
    let mut summary = String::new();
    let mut description = String::new();
    let mut max_len = 0;

    if let Some(media_id) = entry.media_id {
        description = select_media(conn, &media_id)
            .await?
            .description
            .unwrap_or(String::new());
        if description.len() > max_len {
            max_len = description.len();
        }
    }

    if let Some(content_id) = entry.content_id {
        content = select_content(conn, &content_id)
            .await?
            .body
            .unwrap_or(String::new());
        if content.len() > max_len {
            max_len = content.len();
        }
    }

    if let Some(s) = &entry.summary {
        summary = s.to_string();
        if summary.len() > max_len {
            max_len = summary.len();
        }
    }

    let entry_description = Vec::from([content, summary, description])
        .iter()
        .filter(|&f| f.len() >= max_len)
        .cloned()
        .collect::<Vec<String>>()
        .first()
        .cloned()
        .unwrap_or("No entry description found.".to_string());

    Ok(entry_description.clone())
}
