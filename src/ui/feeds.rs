use crate::app::ActiveBlock;
use crate::app::Route;
use crate::app::RouteId;
use crate::db::get_feeds;
use crate::prelude::Feed;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, prelude::*, widgets::ListState};

use super::components::*;
use super::UiCallback;
use super::View;
use super::SELECTED_STYLE;
use super::UNSELECTED_STYLE;

pub struct Feeds {
    list_state: ListState,
    feed_items: Vec<Feed>,
    selected: bool,
}

impl Feeds {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            feed_items: get_feeds().unwrap_or(vec![]),
            selected: false,
        }
    }

    pub fn get_selected_feed(&self) -> Option<Feed> {
        let index = self.list_state.selected()?;
        self.feed_items.get(index).cloned()
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn update_feeds(&mut self) {
        self.feed_items = get_feeds().unwrap_or(vec![]);
    }
}

impl View for Feeds {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let feed_titles: Vec<String> = self
            .feed_items
            .iter()
            .map(|feed| feed.title.clone().unwrap_or("Unnamed Feed".to_string()))
            .collect();

        ItemList::new(&feed_titles)
            .title(Some("Feeds".to_string()))
            .style(match self.selected {
                true => SELECTED_STYLE,
                false => UNSELECTED_STYLE,
            })
            .render(area, buf, &mut self.list_state.clone());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if let Some(index) = self.list_state.selected() {
                    if index < self.feed_items.len() - 1 {
                        self.list_state.select_next();
                    } else {
                        self.list_state.select_first();
                    }
                } else {
                    self.list_state.select_first();
                }

                return Some(Box::new(move |app| {
                    app.ui.update_entries();
                    Ok(())
                }));
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(index) = self.list_state.selected() {
                    if index > 0 {
                        self.list_state.select(Some(index - 1));
                    } else {
                        self.list_state.select(Some(self.feed_items.len() - 1));
                    }
                } else {
                    self.list_state.select(Some(self.feed_items.len() - 1));
                }
                return Some(Box::new(move |app| {
                    app.ui.update_entries();
                    Ok(())
                }));
            }
            KeyCode::Char('l') | KeyCode::Left | KeyCode::Enter => {
                return Some(Box::new(move |app| {
                    app.ui
                        .set_current_route(Route::new(RouteId::Home, ActiveBlock::Entries));
                    Ok(())
                }))
            }
            _ => None,
        }
    }
}
