use crate::app::ActiveBlock;
use crate::app::Route;
use crate::app::RouteId;
use crate::config::Settings;
use crate::data::data::DataEvent;
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
                }

                let feed = self.get_selected_feed().expect("Feed wasn't selected");

                return Some(Box::new(move |app| {
                    app.data_handler
                        .dispatch(DataEvent::ReloadEntries(feed.id.clone()))?;
                    Ok(())
                }));
            }
            KeyCode::Char('k') | KeyCode::Up => {
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

                let feed = self.get_selected_feed().expect("Feed wasn't selected");

                return Some(Box::new(move |app| {
                    app.data_handler
                        .dispatch(DataEvent::ReloadEntries(feed.id.clone()))?;
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
