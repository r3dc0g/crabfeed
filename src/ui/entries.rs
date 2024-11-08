use crate::app::{ActiveBlock, Route, RouteId};
use crate::config::Settings;
use crate::data::data::DataEvent;
use crate::prelude::EntryData;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect, prelude::*, style::Style, widgets::ListState};

use super::util::parse_hex;
use super::View;
use super::{components::*, UiCallback};

pub struct Entries {
    list_state: ListState,
    index: usize,
    entry_items: Vec<Vec<EntryData>>,
    selected: bool,
}

impl Entries {
    pub fn new(entries: Option<Vec<Vec<EntryData>>>) -> Self {
        match entries {
            Some(entry_data) => Self {
                list_state: ListState::default(),
                entry_items: entry_data,
                index: 0,
                selected: false,
            },
            None => Self {
                list_state: ListState::default(),
                entry_items: vec![],
                index: 0,
                selected: false,
            },
        }
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn next_index(&mut self) {
        if !self.entry_items.is_empty() {
            if self.index + 1 > self.entry_items.len() - 1 {
                self.index = 0;
            } else {
                self.index += 1;
            }
        }
    }

    pub fn prev_index(&mut self) {
        if !self.entry_items.is_empty() {
            if self.index == 0 {
                self.index = self.entry_items.len() - 1;
            } else {
                self.index -= 1;
            }
        }
    }

    pub fn update_entries(&mut self, entries: Vec<Vec<EntryData>>) {
        let mut item_groups = entries;
        for group in item_groups.iter_mut() {
            group.reverse();
        }
        self.entry_items = item_groups;
    }
}

impl View for Entries {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings) {
        let primary = parse_hex(&config.colors.primary);
        let selected_style = Style::default().fg(primary);
        let unselected_style = Style::default();

        if self.entry_items.is_empty() {
            let empty_entries: Vec<String> = vec![];
            ItemList::new(&empty_entries)
                .title(Some("Entries (0/0)".to_string()))
                .style(match self.selected {
                    true => selected_style,
                    false => unselected_style,
                })
                .render(area, buf, &mut self.list_state.clone());

            return;
        };

        let items = &self.entry_items[self.index];

        let entries: Vec<(bool, String)> = items
            .iter()
            .map(|entry| (entry.read.clone(), entry.title.clone()))
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
                if !self.entry_items.is_empty() {
                    if !self.entry_items[self.index].is_empty() {
                        if let Some(index) = self.list_state.selected() {
                            if index < self.entry_items[self.index].len() - 1 {
                                self.list_state.select_next();
                            } else {
                                self.list_state.select_first();
                            }
                        } else if self.entry_items[self.index].len() > 0 {
                            self.list_state.select_first();
                        }
                    }
                }

                return None;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !self.entry_items.is_empty() {
                    if !self.entry_items[self.index].is_empty() {
                        if let Some(index) = self.list_state.selected() {
                            if index > 0 {
                                self.list_state.select(Some(index - 1));
                            } else {
                                self.list_state
                                    .select(Some(self.entry_items[self.index].len() - 1));
                            }
                        } else if self.entry_items[self.index].len() > 0 {
                            self.list_state
                                .select(Some(self.entry_items[self.index].len() - 1));
                        }
                    }
                }
                return None;
            }
            KeyCode::Char('l') | KeyCode::Left | KeyCode::Enter => {
                if !self.entry_items.is_empty() {
                    if !self.entry_items[self.index].is_empty() {
                        let entry = Some(
                            self.entry_items[self.index][self.list_state.selected().unwrap_or(0)]
                                .clone(),
                        );

                        self.entry_items[self.index][self.list_state.selected().unwrap()].read =
                            true;
                        let entry_id =
                            self.entry_items[self.index][self.list_state.selected().unwrap()].id;

                        return Some(Box::new(move |app| {
                            app.dispatch(DataEvent::ReadEntry(entry_id))?;
                            app.ui
                                .set_current_route(Route::new(RouteId::Entry, ActiveBlock::Entry));
                            app.ui.set_entry(entry.clone());
                            Ok(())
                        }));
                    }
                }

                None
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
