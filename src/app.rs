use ratatui::layout::Rect;
use std::sync::mpsc::Sender;

use crate::prelude::Feed;

use crate::network::IOEvent;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Feeds,
};

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Feeds,
    Entries,
    Entry,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
}

pub struct App {
    navigation_stack: Vec<Route>,
    pub selected_feed_index: Option<usize>,
    pub selected_entry_index: Option<usize>,
    pub size: Rect,
    pub is_loading: bool,
    pub io_tx: Option<Sender<IOEvent>>,
    pub is_fetching_current_feed: bool,
    pub feeds: Vec<Feed>,
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
            feeds: vec![],
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

    pub fn push_navigation_stack(&mut self, next_route_id: RouteId) {
        if !self.navigation_stack.last().map(|r| r.id == next_route_id).unwrap_or(false) {
            self.navigation_stack.push(Route { id: next_route_id });
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
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    pub fn _update_on_tick(&mut self) {
        // There are no events that happen each tick
        // but there might be in the future ...
    }
}
