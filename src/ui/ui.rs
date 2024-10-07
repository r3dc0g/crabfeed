use crate::app::{Route, RouteId, ActiveBlock};
use crate::network::NetworkEvent;
use crate::prelude::Entry;
use super::{components::*, UiCallback};
use super::entries::Entries;
use super::feeds::Feeds;
use super::entry::Entry as EntryView;
use super::add::Add;
use super::View;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Ui {
    navigation_stack: Vec<Route>,
    pub error_msg: Option<String>,
    pub loading_msg: String,
    pub is_loading: bool,
    feeds: Feeds,
    entries: Entries,
    entry: EntryView,
    add_feed_popup: Add
}

impl Ui {
    pub fn new() -> Self {

        let mut feeds = Feeds::new();
        feeds.select(true);
        let entries = Entries::new(feeds.get_selected_feed().as_ref());
        let entry = EntryView::new(None);
        let add_feed_popup = Add::new();

        Self {
            navigation_stack: vec![Route::default()],
            error_msg: None,
            loading_msg: "Loading...".to_string(),
            is_loading: false,
            feeds,
            entries,
            entry,
            add_feed_popup,
        }
    }

    pub fn get_current_route(&self) -> Option<&Route> {
        self.navigation_stack.last()
    }

    pub fn set_current_route(&mut self, route: Route) {
        self.navigation_stack.push(route);
    }

    pub fn back(&mut self) {
        self.navigation_stack.pop();
    }

    pub fn update_entries(&mut self) {
        let current_feed = self.feeds.get_selected_feed();
        match current_feed {
            Some(feed) => {
                self.entries.update_entries(&feed);
            }
            None => {}
        }
    }

    pub fn update_feeds(&mut self) {
        self.feeds.update_feeds();
    }

    pub fn set_entry(&mut self, entry: Option<Entry>) {
        self.entry.set_entry(entry);
    }

    pub fn update(&mut self) {
        let current_route = self.get_current_route().unwrap_or(&Route::default()).clone();

        self.feeds.select(false);
        self.entries.select(false);

        match current_route.id {
            RouteId::Home => {
                match current_route.active_block {
                    ActiveBlock::Feeds => {
                        self.feeds.select(true);
                    },
                    ActiveBlock::Entries => {
                        self.entries.select(true);
                    },
                    _ => {}
                }
            }
            RouteId::Entry => {}
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key {
            _ if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc => {

                if self.get_current_route().unwrap_or(&Route::default()).id == RouteId::Entry {
                    return self.entry.handle_key_event(key);
                }

                self.back();
                if let None = self.get_current_route() {
                    return Some(Box::new(
                        move |app| {
                            app.is_running = false;
                            Ok(())
                        }
                    ));
                }
                else {
                    return None;
                }
            }
            _ if key.code == KeyCode::Char('a') && key.modifiers == KeyModifiers::CONTROL => {
                self.set_current_route(Route::new(RouteId::Home, ActiveBlock::AddFeed));
                return None;
            }
            _ if key.code == KeyCode::Char('u') && key.modifiers == KeyModifiers::CONTROL => {
                return Some(
                    Box::new(
                        move |app| {
                            app.network_handler.dispatch(NetworkEvent::UpdateFeeds)?;
                            app.is_loading = true;
                            app.ui.is_loading = true;
                            Ok(())
                        }
                    )
                )
            }
            _ => {
                let current_route = self.get_current_route().unwrap_or(&Route::default()).clone();
                match current_route.id {
                    RouteId::Home => {
                        match current_route.active_block {
                            ActiveBlock::Feeds => {
                                self.feeds.handle_key_event(key)
                            }
                            ActiveBlock::Entries => {
                                self.entries.handle_key_event(key)
                            }
                            ActiveBlock::AddFeed => {
                                self.add_feed_popup.handle_key_event(key)
                            }
                            _ => {
                                None
                            }
                        }
                    }
                    RouteId::Entry => {
                        self.entry.handle_key_event(key)
                    }
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
            ]
        )
        .split(area);

        BlockLabel::new()
            .label("Crabfeed".to_string())
            .render(app_layout[0], buf);

        let current_route = self.get_current_route().unwrap_or(&Route::default()).clone();

        match current_route.id {
            RouteId::Home => {

                let lists_section = Layout::new(
                    Direction::Horizontal,
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                )
                .split(app_layout[1]);

                self.feeds.render(lists_section[0], buf);

                self.entries.render(lists_section[1], buf);

                match current_route.active_block {
                    ActiveBlock::AddFeed => {
                        self.add_feed_popup.render(app_layout[1], buf);
                    }
                    _ => {}
                }

            }

            RouteId::Entry => {
                self.entry.render(app_layout[1], buf);
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
