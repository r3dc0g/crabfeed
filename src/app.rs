use ratatui::layout::Rect;
use std::sync::mpsc::Sender;

use crate::control::get_feed;
use crate::db::{get_entries, select_entries, select_feed};
use crate::prelude::{Feed, Entry};

use crate::network::IOEvent;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Feeds,
};

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Entry,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveBlock {
    Feeds,
    Entries,
    Entry,
}

pub struct App {
    navigation_stack: Vec<Route>,
    pub selected_feed_index: Option<usize>,
    pub selected_entry_index: Option<usize>,
    pub size: Rect,
    pub is_loading: bool,
    pub io_tx: Option<Sender<IOEvent>>,
    pub is_fetching_current_feed: bool,
    pub feed_items: Vec<(String, i32)>,
    pub entry_items: Vec<(String, i32)>,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_feed_index: None,
            selected_entry_index: None,
            is_loading: false,
            io_tx: None,
            is_fetching_current_feed: false,
            feed_items: vec![],
            entry_items: vec![],
        }
    }


}

impl App {
    pub fn new(io_tx: Sender<IOEvent>) -> Self {
        Self {
            io_tx: Some(io_tx),
            ..Default::default()
        }
    }

    pub fn dispatch(&mut self, event: IOEvent) {
        self.is_loading = true;
        if let Some(tx) = &self.io_tx {
            if let Err(e) = tx.send(event) {
                self.is_loading = false;
                eprintln!("Error sending IOEvent: {:?}", e);
            };
        }
    }

    pub fn set_feed_items(&mut self, feed_items: Vec<Feed>) {
        self.feed_items = feed_items.iter().map(|f| {
            (f.title.clone().unwrap_or("No title".to_string()).clone(), f.id)
        }).collect();
    }

    pub fn update_entry_items(&mut self, feed_id: i32) {
        let entries = select_entries(feed_id).unwrap_or(vec![]);

        self.entry_items = entries.iter().map(|e| {
            (e.title.clone().unwrap_or("No Title".to_string()), e.id)
        })
        .collect();

    }

    pub fn set_entry_items(&mut self, entry_items: Vec<Entry>) {
        self.entry_items = entry_items.iter().map(|e| {
            (e.title.clone().unwrap_or("No title".to_string()).clone(), e.id)
        })
        .collect();
    }

    pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        if !self.navigation_stack.last().map(|r| r.id == next_route_id).unwrap_or(false) {
            self.navigation_stack.push(Route { id: next_route_id, active_block: next_active_block});
        }
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() > 1 {
            self.navigation_stack.pop()
        } else {
            None
        }
    }

    pub fn get_current_route(&self) -> &Route {
        // self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
        &DEFAULT_ROUTE
    }

    pub fn _update_on_tick(&mut self) {
        // There are no events that happen each tick
        // but there might be in the future ...
    }
}
