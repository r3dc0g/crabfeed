use crate::app::ActiveBlock;
use crate::app::Route;
use crate::app::RouteId;
use crate::config::Settings;
use crate::prelude::FeedData;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
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
    pub fn new(feeds: Vec<FeedData>) -> Self {
        Self {
            list_state: ListState::default(),
            feed_items: feeds,
            selected: false,
        }
    }

    pub fn get_selected_feed(&self) -> Option<FeedData> {
        let index = self.list_state.selected()?;
        self.feed_items.get(index).cloned()
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn update_feeds(&mut self, feeds: Vec<FeedData>) {
        self.feed_items = feeds;
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
