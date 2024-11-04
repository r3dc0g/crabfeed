use core::panic;
use std::{env::current_dir, fs::create_dir_all, time::Duration};

use crabfeed::{
    config::{get_configuration, Settings},
    data::data::{DataEvent, DataHandler},
    ui::util::parse_hex,
};
use env_logger::Target;
use log::{debug, info};
use ratatui::style::Color;
use tokio::time::sleep;
use uuid::Uuid;

#[tokio::test]
async fn feeds_are_reloaded() {
    // Start logger
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle a reload event
    DataHandler::handle_event(db_url, DataEvent::ReloadFeeds, sender)
        .await
        .expect("Failed to handle ReloadFeeds event");

    sleep(Duration::from_secs(2)).await;

    // Assert we get an event back
    let event = receiver
        .try_recv()
        .expect("Failed to receive ReloadFeeds(_) event");

    match event {
        DataEvent::ReloadedFeeds(feeds) => {
            // Assert feeds were pulled, but none reside in the db
            assert_eq!(feeds.len(), 0);
        }
        DataEvent::Error(e) => {
            panic!("{}", e);
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }
}

#[tokio::test]
async fn feed_is_added() {
    // Start logger
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle an insertion event
    DataHandler::handle_event(
        db_url,
        DataEvent::AddFeed("https://archlinux.org/feeds/news/".to_string()),
        sender,
    )
    .await
    .expect("Failed to handle AddFeed event");

    sleep(Duration::from_secs(2)).await;

    let event = receiver
        .try_recv()
        .expect("Failed to receive response DataEvent");

    match event {
        DataEvent::Complete => {}
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }
}

#[tokio::test]
async fn entries_are_reloaded() {
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle an insertion event
    DataHandler::handle_event(
        db_url.clone(),
        DataEvent::AddFeed("https://archlinux.org/feeds/news/".to_string()),
        sender.clone(),
    )
    .await
    .expect("Failed to handle AddFeed event");

    sleep(Duration::from_secs(2)).await;

    let event = receiver
        .try_recv()
        .expect("Failed to receive response DataEvent");

    match event {
        DataEvent::Complete => {}
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    DataHandler::handle_event(db_url.clone(), DataEvent::ReloadFeeds, sender.clone())
        .await
        .expect("Failed to handle ReloadEntries event");

    let mut feeds = vec![];

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive response DataEvent")
    {
        DataEvent::ReloadedFeeds(data) => {
            assert_eq!(data.len(), 1);
            for feed in data {
                feeds.push(feed);
            }
        }
        DataEvent::Error(e) => {
            panic!("Error: {e}");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    let feed_ids: Vec<i64> = feeds.iter().map(|f| f.id).collect();

    DataHandler::handle_event(
        db_url.clone(),
        DataEvent::ReloadEntries(feed_ids.clone()),
        sender.clone(),
    )
    .await
    .expect("Failed to handle ReloadEntries event");

    let mut entries = vec![];

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive response DataEvent")
    {
        DataEvent::ReloadedEntries(data) => {
            assert_eq!(data.len(), 1);
            assert_eq!(data[0].len(), 10);
            for group in data {
                for data in group {
                    entries.push(data);
                }
            }
        }
        DataEvent::Error(e) => {
            panic!("Error: {e}");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        DataEvent::Complete => {
            debug!("ReloadEntries completed");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }
}

#[tokio::test]
async fn feed_is_deleted() {
    // Start logger
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle an insertion event
    DataHandler::handle_event(
        db_url.clone(),
        DataEvent::AddFeed("https://archlinux.org/feeds/news/".to_string()),
        sender.clone(),
    )
    .await
    .expect("Failed to handle AddFeed event");

    sleep(Duration::from_secs(2)).await;

    let completed_event = receiver
        .try_recv()
        .expect("Failed to receive response DataEvent");

    match completed_event {
        DataEvent::Complete => {}
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Handle reload event
    DataHandler::handle_event(db_url.clone(), DataEvent::ReloadFeeds, sender.clone())
        .await
        .expect("Failed to handle ReloadFeeds event");

    let mut feeds = vec![];

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive response DataEvent")
    {
        DataEvent::ReloadedFeeds(data) => {
            assert_eq!(data.len(), 1);
            for feed in data {
                feeds.push(feed);
            }
        }
        DataEvent::Error(e) => {
            panic!("Error: {e}");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    let feed_ids: Vec<i64> = feeds.iter().map(|f| f.id).collect();

    DataHandler::handle_event(
        db_url.clone(),
        DataEvent::ReloadEntries(feed_ids.clone()),
        sender.clone(),
    )
    .await
    .expect("Failed to handle ReloadEntries event");

    let mut entries = vec![];

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive response DataEvent")
    {
        DataEvent::ReloadedEntries(data) => {
            assert_eq!(data.len(), 1);
            assert_eq!(data[0].len(), 10);
            for group in data {
                for data in group {
                    entries.push(data);
                }
            }
        }
        DataEvent::Error(e) => {
            panic!("Error: {e}");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        DataEvent::Complete => {
            debug!("ReloadEntries completed");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    // Handle delete event
    DataHandler::handle_event(db_url.clone(), DataEvent::DeleteFeed(feed_ids[0]), sender)
        .await
        .expect("Failed to handle DeleteFeed event");

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        DataEvent::Deleting(msg) => {
            println!("{msg}");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        DataEvent::Complete => {
            debug!("Delete feed completed");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }
}

#[test]
fn configuration_is_found() {
    let config = get_configuration().unwrap();

    assert_ne!(config, Settings::default());
}

#[test]
fn hex_code_string_returns_rgb() {
    let hex = "#00ff00".to_string();
    let color = parse_hex(&hex);
    assert_eq!(color, Color::Rgb(0, 255, 0));
}

fn init_logger() {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .is_test(true)
        .try_init();
}

fn get_test_database_url() -> String {
    let curr_dir = current_dir()
        .expect("Coudn't get current directory")
        .display()
        .to_string();
    create_dir_all(format!("{curr_dir}/tests/test_db")).expect("Failed to create test_db folder");
    let db_name = Uuid::new_v4().to_string();

    let url = format!("sqlite:///{curr_dir}/tests/test_db/{db_name}.db");
    info!("Database url: {url}");
    url
}
