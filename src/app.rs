use ratatui::layout::Rect;
use std::sync::mpsc::Sender;

use crate::db::{delete_feed, find_entry_links, get_feeds, select_entries, select_entry};
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

impl Default for RouteId {
    fn default() -> Self {
        RouteId::Home
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveBlock {
    Feeds,
    Entries,
    Entry,
    Input,
}

impl Default for ActiveBlock {
    fn default() -> Self {
        ActiveBlock::Feeds
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

pub struct App {
    navigation_stack: Vec<Route>,
    pub selected_feed_index: Option<usize>,
    pub selected_entry_index: Option<usize>,
    pub entry_line_index: u16,
    pub size: Rect,
    pub is_loading: bool,
    pub input: Vec<char>,
    pub input_cursor_position: usize,
    pub input_i: usize,
    pub io_tx: Option<Sender<IOEvent>>,
    pub is_fetching_current_feed: bool,
    pub feed_items: Vec<(String, i32)>,
    pub entry_items: Vec<(String, (i32, bool))>,
    pub entry: Option<Entry>,
    pub link_items: Vec<(String, i32)>,
    pub error_msg: Option<String>,
    pub total_entries: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_feed_index: None,
            selected_entry_index: None,
            entry_line_index: 0,
            is_loading: false,
            input: vec![],
            input_cursor_position: 0,
            input_i: 0,
            io_tx: None,
            is_fetching_current_feed: false,
            feed_items: vec![],
            entry_items: vec![],
            entry: None,
            link_items: vec![],
            error_msg: None,
            total_entries: 0,
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

    pub fn update_feed_items(&mut self) {
        let index = self.selected_feed_index.unwrap_or(0);
        self.is_loading = true;
        if let Ok(feeds) = get_feeds() {
            self.feed_items = feeds.iter().map(|f| {
                (f.title.clone().unwrap_or("No title".to_string()).clone(), f.id)
            }).collect();
            if index < self.feed_items.len() {
                self.selected_feed_index = Some(index);
            }
            else {
                self.selected_feed_index = None;
            }
            self.selected_feed_index = None;
        }
        self.update_entry_items(0);
        self.is_loading = false;
    }

    pub fn update_entry_items(&mut self, feed_id: i32) {
        let entries = select_entries(feed_id).unwrap_or(vec![]);
        let index = self.selected_entry_index;
        self.entry_items = entries.iter().rev().map(|e| {
            (e.title.clone().unwrap_or("No Title".to_string()), (e.id, e.read.unwrap_or(false)))
        })
        .collect();
        if let Some(index) = index {
            if index < self.entry_items.len() {
                self.selected_entry_index = Some(index);
            }
            else {
                self.selected_entry_index = None;
            }
        }
        else {
            self.selected_entry_index = None;
        }

        self.total_entries = self.entry_items.len();
    }

    pub fn set_entry(&mut self, entry_id: i32) {
        if let Ok(entry) = select_entry(&entry_id) {
            self.entry = Some(entry);
            return;
        }

        self.entry = None;
    }

    pub fn update_link_items(&mut self, entry_id: i32) {
        let links = find_entry_links(entry_id).unwrap_or(vec![]);

        self.link_items = links.iter().map(|l| {
            (l.href.clone(), l.id)
        })
        .collect();
    }

    pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        self.navigation_stack.push(Route { id: next_route_id, active_block: next_active_block});
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() > 1 {
            self.navigation_stack.pop()
        } else {
            None
        }
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    pub fn set_current_route(&mut self, route: RouteId, active_block: ActiveBlock) {
        self.push_navigation_stack(route, active_block);
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor_position = 0;
        self.input_i = 0;
    }

    pub fn delete_feed(&mut self) {
        if let Some(feed_index) = self.selected_feed_index {
            let feed_id = self.feed_items[feed_index].1;

            if let Err(e) = delete_feed(feed_id) {
                self.error_msg = Some(format!("Error deleting feed: {:?}", e));
                return;
            };

            self.feed_items.remove(feed_index);
            self.update_feed_items();
        }
    }

    pub fn _update_on_tick(&mut self) {
        // There are no events that happen each tick
        // but there might be in the future ...
    }
}
