use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect,
    widgets::ListState
};
use crate::db::get_feeds;
use crate::prelude::Feed;

use super::components::*;
use super::UiCallback;
use super::View;

pub struct Feeds {
    list_state: ListState,
    feed_items: Vec<Feed>,
}

impl View for Feeds {
    fn render(&self, area: Rect, buf: &mut Buffer) {

        let feed_titles: Vec<String> = self.feed_items.iter().map(|feed| feed.title.clone().unwrap_or("Unnamed Feed".to_string())).collect();

        ItemList::new(&feed_titles)
            .title(Some("Feeds".to_string()))
            .render(area, buf, &mut self.list_state.clone());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        None
    }

}

impl Feeds {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            feed_items: get_feeds().unwrap_or(vec![]),
        }
    }

    pub fn get_selected_feed(&self) -> Option<Feed> {
        let index = self.list_state.selected()?;
        self.feed_items.get(index).cloned()
    }
}
