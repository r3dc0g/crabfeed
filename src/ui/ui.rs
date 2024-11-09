use super::add::Add;
use super::entries::Entries;
use super::entry::Entry as EntryView;
use super::feeds::Feeds;
use super::View;
use super::{components::*, UiCallback};
use crate::app::{ActiveBlock, Route, RouteId};
use crate::config::Settings;
use crate::data::data::DataEvent;
use crate::prelude::{EntryData, FeedData};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::*;

pub struct Ui {
    navigation_stack: Vec<Route>,
    pub error_msg: Option<String>,
    pub loading_msg: String,
    pub is_loading: bool,
    feeds: Feeds,
    entries: Entries,
    entry: EntryView,
    popup: Option<Box<dyn View>>,
    config: Settings,
}

impl Ui {
    pub fn new(config: Settings) -> Self {
        let mut feeds = Feeds::new(None);
        feeds.select(true);
        let entries = Entries::new(None);
        let entry = EntryView::new(None);

        Self {
            navigation_stack: vec![Route::default()],
            error_msg: None,
            loading_msg: "Loading...".to_string(),
            is_loading: false,
            feeds,
            entries,
            entry,
            popup: None,
            config,
        }
    }

    pub fn get_current_route(&self) -> Option<&Route> {
        self.navigation_stack.last()
    }

    pub fn set_current_route(&mut self, route: Route) {
        self.navigation_stack.push(route);
    }

    pub fn back(&mut self) {
        if self.popup.is_some() {
            self.unset_popup();
            return;
        }
        self.navigation_stack.pop();
    }

    pub fn update_entries(&mut self, entries: Vec<Vec<EntryData>>) {
        self.entries.update_entries(entries);
    }

    pub fn next_entries(&mut self) {
        self.entries.next_index();
    }

    pub fn prev_entries(&mut self) {
        self.entries.prev_index();
    }

    pub fn remove_entries(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn update_feeds(&mut self, feeds: Vec<FeedData>) {
        self.feeds.update_feeds(feeds);
    }

    pub fn set_entry(&mut self, entry: Option<EntryData>) {
        if let Some(data) = entry {
            self.entry.set_entry(data);
        }
    }

    pub fn unset_popup(&mut self) {
        self.popup = None;
    }

    pub fn update(&mut self) {
        let current_route = self
            .get_current_route()
            .unwrap_or(&Route::default())
            .clone();

        self.feeds.select(false);
        self.entries.select(false);

        match current_route.id {
            RouteId::Home => match current_route.active_block {
                ActiveBlock::Feeds => {
                    self.feeds.select(true);
                }
                ActiveBlock::Entries => {
                    self.entries.select(true);
                }
                _ => {}
            },
            RouteId::Entry => {}
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key {
            _ if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc => {
                if let Some(popup) = &mut self.popup {
                    return popup.handle_key_event(key);
                }

                if self.get_current_route().unwrap_or(&Route::default()).id == RouteId::Entry {
                    return self.entry.handle_key_event(key);
                }

                self.back();
                if let None = self.get_current_route() {
                    return Some(Box::new(move |app| {
                        app.is_running = false;
                        Ok(())
                    }));
                } else {
                    return None;
                }
            }
            _ if key.code == KeyCode::Char('a') && key.modifiers == KeyModifiers::CONTROL => {
                self.popup = Some(Box::new(Add::new()));
                return None;
            }
            _ if key.code == KeyCode::Char('u') && key.modifiers == KeyModifiers::CONTROL => {
                return Some(Box::new(move |app| {
                    app.data_handler.dispatch(DataEvent::UpdateFeeds)?;
                    app.is_loading = true;
                    app.ui.is_loading = true;
                    Ok(())
                }))
            }
            _ => {
                let current_route = self
                    .get_current_route()
                    .unwrap_or(&Route::default())
                    .clone();
                if let Some(popup) = &mut self.popup {
                    return popup.handle_key_event(key);
                }
                match current_route.id {
                    RouteId::Home => match current_route.active_block {
                        ActiveBlock::Feeds => self.feeds.handle_key_event(key),
                        ActiveBlock::Entries => self.entries.handle_key_event(key),
                        _ => None,
                    },
                    RouteId::Entry => self.entry.handle_key_event(key),
                }
            }
        }
    }
}

impl Widget for &mut Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(3),
                Constraint::Max(100),
                Constraint::Length(3),
            ],
        )
        .split(area);

        BlockLabel::new()
            .label("Crabfeed".to_string())
            .render(app_layout[0], buf);

        let current_route = self
            .get_current_route()
            .unwrap_or(&Route::default())
            .clone();

        match current_route.id {
            RouteId::Home => {
                if area.height > (area.width as f32 * 0.5) as u16 {
                    match current_route.active_block {
                        ActiveBlock::Feeds => {
                            self.feeds.render(app_layout[1], buf, &self.config);
                        }
                        ActiveBlock::Entries => {
                            self.entries.render(app_layout[1], buf, &self.config);
                        }
                        _ => {}
                    }
                } else {
                    let lists_section = Layout::new(
                        Direction::Horizontal,
                        [Constraint::Percentage(50), Constraint::Percentage(50)],
                    )
                    .split(app_layout[1]);

                    self.feeds.render(lists_section[0], buf, &self.config);

                    self.entries.render(lists_section[1], buf, &self.config);
                }

                if let Some(popup) = &self.popup {
                    popup.render(app_layout[1], buf, &self.config);
                }
            }

            RouteId::Entry => {
                self.entry.render(app_layout[1], buf, &self.config);
            }
        }

        if self.is_loading {
            BlockLabel::new()
                .label(self.loading_msg.clone())
                .render(app_layout[2], buf);
        } else {
            BlockLabel::new()
                .label("Ctrl+a to add feed, Ctrl+d to delete feed, (ESC/Q) to quit".to_string())
                .render(app_layout[2], buf);
        }
    }
}
