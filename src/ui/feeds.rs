use crate::app::ActiveBlock;
use crate::app::Route;
use crate::app::RouteId;
use crate::config::Settings;
use crate::prelude::FeedData;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use ratatui::{buffer::Buffer, layout::Rect, prelude::*, widgets::ListState};

use super::components::*;
use super::util::parse_hex;
use super::UiCallback;
use super::View;

pub struct Feeds {
    list_state: ListState,
    feed_items: Vec<FeedData>,
    selected: bool,
}

impl Feeds {
    pub fn new(feeds: Option<Vec<FeedData>>) -> Self {
        match feeds {
            Some(feed_data) => Self {
                list_state: ListState::default(),
                feed_items: feed_data,
                selected: false,
            },
            None => Self {
                list_state: ListState::default(),
                feed_items: vec![],
                selected: false,
            },
        }
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn update_feeds(&mut self, feeds: Vec<FeedData>) {
        self.feed_items = feeds;
    }

    fn scroll_down(&mut self) -> Option<UiCallback> {
        if self.feed_items.is_empty() {
            return None;
        }

        if let Some(index) = self.list_state.selected() {
            if index < self.feed_items.len() - 1 {
                self.list_state.select_next();
            } else {
                self.list_state.select_first();
            }
        } else {
            self.list_state.select_first();
            return None;
        }

        return Some(Box::new(move |app| {
            app.ui.next_entries();
            Ok(())
        }));
    }

    fn scroll_up(&mut self) -> Option<UiCallback> {
        if self.feed_items.is_empty() {
            return None;
        }

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
            app.ui.prev_entries();
            Ok(())
        }));
    }

    fn select_feed(&mut self) -> Option<UiCallback> {
        return Some(Box::new(move |app| {
            app.ui
                .set_current_route(Route::new(RouteId::Home, ActiveBlock::Entries));
            Ok(())
        }));
    }

    fn delete_feed(&mut self) -> Option<UiCallback> {
        if let Some(index) = self.list_state.selected() {
            let feed_id = self.feed_items[index].id;
            self.feed_items.remove(index);

            if index > 0 {
                self.list_state.select(Some(index - 1));
            } else {
                self.list_state.select(None);
            }

            return Some(Box::new(move |app| {
                app.ui.remove_entries(index);
                app.dispatch(crate::data::data::DataEvent::DeleteFeed(feed_id.clone()))?;
                Ok(())
            }));
        }

        return None;
    }
}

impl View for Feeds {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings) {
        let primary = parse_hex(&config.colors.primary);

        let selected_style = Style::default().fg(primary);
        let unselected_style = Style::default();

        let feed_titles: Vec<String> = self
            .feed_items
            .iter()
            .map(|feed| feed.title.clone())
            .collect();

        ItemList::new(&feed_titles)
            .title(Some("Feeds".to_string()))
            .style(match self.selected {
                true => selected_style,
                false => unselected_style,
            })
            .render(area, buf, &mut self.list_state.clone());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
            KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
            KeyCode::Char('l') | KeyCode::Left | KeyCode::Enter => {
                // Don't select a phantom feed and move to entries
                if key.code == KeyCode::Enter && self.feed_items.is_empty() {
                    return None;
                }

                self.select_feed()
            }
            _ if key.code == KeyCode::Char('d') && key.modifiers == KeyModifiers::CONTROL => {
                self.delete_feed()
            }
            _ => None,
        }
    }
}
