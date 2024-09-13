mod tuihtml;

use crate::db::*;
use crate::prelude::Entry;
use crate::network::IOEvent;

use ratatui::widgets::Paragraph;
use ratatui::{
    layout::Rect,
    widgets::ListState,
};
use std::sync::mpsc::Sender;

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
    pub feed_list_state: ListState,
    pub entry_list_state: ListState,
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
    pub loading_msg: String,
    pub total_entries: usize,
    pub entry_content: Option<Paragraph<'static>>,
    pub entry_summary: Option<Paragraph<'static>>,
    pub entry_description: Option<Paragraph<'static>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            feed_list_state: ListState::default(),
            entry_list_state: ListState::default(),
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
            loading_msg: "Loading...".to_string(),
            total_entries: 0,
            entry_content: None,
            entry_summary: None,
            entry_description: None,
        }
    }


}

impl App {
    pub fn new(io_tx: Sender<IOEvent>) -> Self {
        Self {
            io_tx: Some(io_tx),
            ..Self::default()
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
        let index = self.feed_list_state.selected().unwrap_or(0);
        self.loading_msg = "Reloading Feed Items...".to_string();
        self.is_loading = true;
        let feeds = get_feeds().unwrap_or(vec![]);
        self.feed_items = feeds.iter().map(|f| {
            (f.title.clone().unwrap_or("No title".to_string()).clone(), f.id)
        }).collect();
        if index < self.feed_items.len() {
            self.feed_list_state.select(Some(index));
        }
        else {
            self.feed_list_state.select(None);
        }
        self.feed_list_state.select(None);
        self.update_entry_items(0);
        self.is_loading = false;
    }

    pub fn update_entry_items(&mut self, feed_id: i32) {
        let entries = select_entries(feed_id).unwrap_or(vec![]);
        let index = self.entry_list_state.selected();
        self.entry_items = entries.iter().rev().map(|e| {
            (e.title.clone().unwrap_or("No Title".to_string()), (e.id, e.read.unwrap_or(false)))
        })
        .collect();
        if let Some(index) = index {
            if index < self.entry_items.len() {
                self.entry_list_state.select(Some(index));
            }
            else {
                self.entry_list_state.select(None);
            }
        }
        else {
            self.entry_list_state.select(None);
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

    pub fn set_content(&mut self, content: Option<i32>) {

        match content {
            Some(content_id) => {
                if let Ok(content) = select_content(&content_id) {
                    let content_html = content.body.clone().unwrap_or("".to_string());

                    if let Ok(tui_content) = tuihtml::parse_html(content_html) {
                        self.entry_content = Some(tui_content);
                    }
                    else {
                        self.entry_content = None;
                    }
                }
            }

            None => {
                self.entry_content = None;
            }
        }

    }

    pub fn set_summary(&mut self) {

        if let Some(entry) = &self.entry {
            if let Some(summary_html) = &entry.summary {
                if let Ok(tui_summary) = tuihtml::parse_html(summary_html.to_string()) {
                    self.entry_summary = Some(tui_summary);
                }
                else {
                    self.entry_summary = None;
                }
            }
            else {
                self.entry_summary = None;
            }
        }
        else {
            self.entry_summary = None;
        }

    }

    pub fn set_entry_description(&mut self) {

        if let Some(entry) = &self.entry {
            if let Some(media_id) = entry.media_id {
                if let Ok(media) = select_media(media_id) {
                    if let Some(description) = media.description {
                        if let Ok(tui_description) = tuihtml::parse_html(description) {
                            self.entry_description = Some(tui_description);
                        }
                        else {
                            self.entry_description = None;
                        }
                    }
                    else {
                        self.entry_description = None;
                    }
                }
                else {
                    self.entry_description = None;
                }
            }
            else {
                self.entry_description = None;
            }
        }
        else {
            self.entry_description = None;
        }
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

}
