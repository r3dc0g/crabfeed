use crate::app::{ActiveBlock, DEFAULT_ROUTE};
use crate::app::{Route, RouteId};
use super::{components::*, UiCallback};
use super::entries::Entries;
use super::feeds::Feeds;
use super::entry::Entry as EntryView;
use super::View;

use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Ui {
    navigation_stack: Vec<Route>,

    // For input text area
    // pub input: Vec<char>,
    // pub input_cursor_position: usize,
    // pub input_i: usize,
    pub error_msg: Option<String>,
    pub loading_msg: String,
    feeds: Feeds,
    entries: Entries,
    entry: EntryView,
}

impl Ui {
    pub fn new() -> Self {

        let feeds = Feeds::new();
        let entries = Entries::new(feeds.get_selected_feed().as_ref());
        let entry = EntryView::new();

        Self {
            navigation_stack: Vec::new(),
            error_msg: None,
            loading_msg: "Loading...".to_string(),
            feeds,
            entries,
            entry,
        }
    }

    pub fn get_current_route(&self) -> Option<&Route> {
        self.navigation_stack.last()
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        let current_route = self.get_current_route().unwrap_or(&DEFAULT_ROUTE);
        match current_route.id {
            RouteId::Home => {
                match current_route.active_block {
                    ActiveBlock::Feeds => {
                        self.feeds.handle_key_event(key)
                    }
                    ActiveBlock::Entries => {
                        self.entries.handle_key_event(key)
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

        match self.get_current_route().unwrap_or(&DEFAULT_ROUTE).id {
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

            }

            RouteId::Entry => {
                self.entry.render(app_layout[1], buf);
            }
        }
    }
}
