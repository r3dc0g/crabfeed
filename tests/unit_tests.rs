use core::panic;
use std::{env::current_dir, fs::create_dir_all, time::Duration};

use chrono::Utc;
use crabfeed::{
    app::AppEvent,
    config::{get_configuration, Settings},
    data::{
        data::{self, DataEvent},
        db::{connect, select_entry},
    },
    ui::util::parse_hex,
};
use env_logger::Target;
use log::{debug, info};
use ratatui::style::Color;
use tokio::time::sleep;

#[tokio::test]
async fn data_is_refreshed() {
    // Start logger
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle a reload event
    data::handle_event(db_url, DataEvent::Refresh, sender)
        .await
        .expect("Failed to handle ReloadFeeds event");

    sleep(Duration::from_secs(2)).await;

    // Assert we get an event back
    let event = receiver
        .try_recv()
        .expect("Failed to receive ReloadFeeds(_) event");

    match event {
        AppEvent::FeshData(data) => {
            // Assert feeds were pulled, but none reside in the db
            assert_eq!(data.feeds.len(), 0);
        }
        AppEvent::Error(e) => {
            panic!("{}", e);
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Assert we get an event back
    let complete = receiver
        .try_recv()
        .expect("Failed to receive ReloadFeeds(_) event");

    match complete {
        AppEvent::Complete => {}
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

    // List of different feeds to test
    let feed_list = [
        "https://archlinux.org/feeds/news/".to_string(),
        "https://100r.co/links/rss.xml".to_string(),
        "https://guerrillahistory.libsyn.com/rss".to_string(),
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCUBsjvdHcwZd3ztdY1Zadcw".to_string(),
        "https://manga4life.com/rss/Tengoku-Daimakyou.xml".to_string(),
    ];

    for feed in feed_list {
        // Handle an insertion event
        data::handle_event(db_url.clone(), DataEvent::AddFeed(feed), sender.clone())
            .await
            .expect("Failed to handle AddFeed event");

        sleep(Duration::from_secs(2)).await;

        let event = receiver
            .try_recv()
            .expect("Failed to receive response DataEvent");

        match event {
            AppEvent::Complete => {}
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
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

    // List of different feeds to test
    let feed_list = [
        "https://archlinux.org/feeds/news/".to_string(),
        "https://100r.co/links/rss.xml".to_string(),
        "https://guerrillahistory.libsyn.com/rss".to_string(),
        "https://www.youtube.com/feeds/videos.xml?channel_id=UCUBsjvdHcwZd3ztdY1Zadcw".to_string(),
        "https://manga4life.com/rss/Tengoku-Daimakyou.xml".to_string(),
    ];

    for feed in feed_list {
        // Handle an insertion event
        data::handle_event(db_url.clone(), DataEvent::AddFeed(feed), sender.clone())
            .await
            .expect("Failed to handle AddFeed event");

        sleep(Duration::from_secs(2)).await;

        match receiver
            .try_recv()
            .expect("Failed to receive response DataEvent")
        {
            AppEvent::Complete => {}
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
        }

        sleep(Duration::from_secs(2)).await;

        // Handle a reload event
        data::handle_event(db_url.clone(), DataEvent::Refresh, sender.clone())
            .await
            .expect("Failed to handle ReloadFeeds event");

        sleep(Duration::from_secs(2)).await;

        let mut feeds = vec![];

        match receiver
            .try_recv()
            .expect("Failed to receive ReloadFeeds(_) event")
        {
            AppEvent::FeshData(data) => {
                // Assert data was pulled
                assert_eq!(data.feeds.len(), 1);

                for feed in data.feeds {
                    feeds.push(feed);
                }
            }
            AppEvent::Error(e) => {
                panic!("{}", e);
            }
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
        }

        match receiver
            .try_recv()
            .expect("Failed to receive completed response DataEvent")
        {
            AppEvent::Complete => {
                debug!("ReloadEntries completed");
            }
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
        }

        let feed_ids: Vec<i64> = feeds.iter().map(|f| f.id).collect();

        // Handle delete event
        data::handle_event(
            db_url.clone(),
            DataEvent::DeleteFeed(feed_ids[0]),
            sender.clone(),
        )
        .await
        .expect("Failed to handle DeleteFeed event");

        sleep(Duration::from_secs(2)).await;

        match receiver
            .try_recv()
            .expect("Failed to receive completed response DataEvent")
        {
            AppEvent::Deleting(msg) => {
                debug!("{msg}");
            }
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
        }

        match receiver
            .try_recv()
            .expect("Failed to receive completed response DataEvent")
        {
            AppEvent::Complete => {
                debug!("Delete feed completed");
            }
            e => {
                panic!("Unexpected event received, {:?}", e);
            }
        }
    }
}

#[tokio::test]
async fn entry_is_marked_read() {
    // Start logger
    init_logger();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);

    // Get fresh test data base
    let db_url = get_test_database_url();

    // Handle an insertion event
    data::handle_event(
        db_url.clone(),
        DataEvent::AddFeed("https://archlinux.org/feeds/news/".to_string()),
        sender.clone(),
    )
    .await
    .expect("Failed to handle AddFeed event");

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive response DataEvent")
    {
        AppEvent::Complete => {}
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Handle a reload event
    data::handle_event(db_url.clone(), DataEvent::Refresh, sender.clone())
        .await
        .expect("Failed to handle ReloadFeeds event");

    sleep(Duration::from_secs(2)).await;

    let mut feeds = vec![];
    let mut entries = vec![];

    match receiver
        .try_recv()
        .expect("Failed to receive ReloadFeeds(_) event")
    {
        AppEvent::FeshData(data) => {
            // Assert data was pulled
            assert_eq!(data.feeds.len(), 1);
            assert_eq!(data.entries[0].len(), 10);

            for feed in data.feeds {
                feeds.push(feed);
            }

            for entry in data.entries[0].clone() {
                entries.push(entry);
            }
        }
        AppEvent::Error(e) => {
            panic!("{}", e);
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        AppEvent::Complete => {
            debug!("ReloadEntries completed");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    let entry_id = entries[0].id;

    data::handle_event(
        db_url.clone(),
        DataEvent::ReadEntry(entry_id),
        sender.clone(),
    )
    .await
    .expect("Failed to send ReadEntry event");

    sleep(Duration::from_secs(2)).await;

    match receiver
        .try_recv()
        .expect("Failed to receive completed response DataEvent")
    {
        AppEvent::Complete => {
            debug!("ReloadEntries completed");
        }
        e => {
            panic!("Unexpected event received, {:?}", e);
        }
    }

    let conn = &mut connect(db_url.clone())
        .await
        .expect("Failed to connect to database");

    let entry = select_entry(conn, &entry_id)
        .await
        .expect("Failed to get entry from database");

    assert_eq!(entry.read, Some(true));
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
    let db_name = Utc::now().to_string();

    let url = format!("sqlite:///{curr_dir}/tests/test_db/{db_name}.db");
    info!("Database url: {url}");
    url
}
