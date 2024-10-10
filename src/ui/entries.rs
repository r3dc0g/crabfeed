use crate::app::{ActiveBlock, Route, RouteId};
use crate::config::Settings;
use crate::db::{get_entries, mark_entry_read};
use crate::prelude::{Entry, Feed};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect, prelude::*, style::Style, widgets::ListState};

use super::util::parse_hex;
use super::View;
use super::{components::*, UiCallback};

pub struct Entries {
    list_state: ListState,
    entry_items: Vec<Entry>,
    selected: bool,
}

impl Entries {
    pub fn new(selected_feed: Option<&Feed>) -> Self {
        match selected_feed {
            Some(feed) => Self {
                list_state: ListState::default(),
                entry_items: get_entries(feed).unwrap_or(vec![]),
                selected: false,
            },
            None => Self {
                list_state: ListState::default(),
                entry_items: vec![],
                selected: false,
            },
        }
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn update_entries(&mut self, feed: &Feed) {
        let mut items = get_entries(feed).unwrap_or(vec![]);
        items.reverse();
        self.entry_items = items;
    }

    pub fn reset(&mut self) {
        self.list_state.select(None);
    }
}

impl View for Entries {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings) {

        let primary = parse_hex(&config.colors.primary);
        let selected_style = Style::default().fg(primary);
        let unselected_style = Style::default();

        let entries: Vec<(bool, String)> = self
            .entry_items
            .iter()
            .map(|entry| {
                (
                    entry.read.clone().unwrap_or(false),
                    entry.title.clone().unwrap_or("Untitled Entry".to_string()),
                )
            })
            .collect();
        let list_len = entries.len();
        let mut unread_len = 0;
        let unread_marker = "*";
        let mut lines = vec![];

        for i in 0..list_len {
            let mut read_style = Style::default();

            let has_read = entries
                .get(i)
                .expect("Error: More read items than entry items")
                .0;

            if !has_read {
                read_style = read_style.bold();
                unread_len += 1;
                let curr_title = entries
                    .get(i)
                    .expect("Error: Invalid title length")
                    .1
                    .clone();
                let new_title = format!("{} {}", unread_marker, curr_title);
                let line = Line::styled(new_title, read_style);
                lines.push(line);
            } else {
                let curr_title = entries
                    .get(i)
                    .expect("Error: Invalid title length")
                    .1
                    .clone();
                let new_title = format!("- {}", curr_title);
                let line = Line::styled(new_title, read_style);
                lines.push(line);
            }
        }

        ItemList::new(&lines)
            .title(Some(format!("Entries ({}/{})", unread_len, list_len)))
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
                    if index < self.entry_items.len() - 1 {
                        self.list_state.select_next();
                    } else {
                        self.list_state.select_first();
                    }
                } else if self.entry_items.len() > 0 {
                    self.list_state.select_first();
                }

                return None;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(index) = self.list_state.selected() {
                    if index > 0 {
                        self.list_state.select(Some(index - 1));
                    } else {
                        self.list_state.select(Some(self.entry_items.len() - 1));
                    }
                } else if self.entry_items.len() > 0 {
                    self.list_state.select(Some(self.entry_items.len() - 1));
                }
                return None;
            }
            KeyCode::Char('l') | KeyCode::Left | KeyCode::Enter => {
                let entry = Some(self.entry_items[self.list_state.selected().unwrap_or(0)].clone());
                if let Some(ref real_entry) = entry {
                    if let Err(_) = mark_entry_read(real_entry.id) {
                        // TODO: Error Handling
                    }
                }
                return Some(Box::new(move |app| {
                    app.ui
                        .set_current_route(Route::new(RouteId::Entry, ActiveBlock::Entry));
                    app.ui.set_entry(entry.clone());
                    Ok(())
                }));
            }
            KeyCode::Char('h') | KeyCode::Right => {
                return Some(Box::new(move |app| {
                    app.ui.back();
                    Ok(())
                }))
            }
            _ => None,
        }
    }
}
