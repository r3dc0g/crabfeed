use crossterm::event::KeyEvent;
use ratatui::style::Stylize;
use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect, style::Style,
    widgets::ListState
};
use crate::db::get_entries;
use crate::prelude::{Entry, Feed};

use super::{components::*, UiCallback};
use super::View;

pub struct Entries {
    list_state: ListState,
    entry_items: Vec<Entry>,
}

impl View for Entries {
    fn render(&self, area: Rect, buf: &mut Buffer) {

        let entries: Vec<(bool, String)> = self.entry_items.iter().map(|entry| (entry.read.clone().unwrap_or(false), entry.title.clone().unwrap_or("Untitled Entry".to_string()))).collect();
        let list_len  = entries.len();
        let mut unread_len = 0;
        let unread_marker = "*";
        let mut lines = vec![];

        for i in 0..list_len {

            let mut read_style = Style::default();

            let has_read = entries.get(i).expect("Error: More read items than entry items").0;

            if !has_read {
                read_style = read_style.bold();
                unread_len += 1;
                let curr_title = entries.get(i).expect("Error: Invalid title length").1.clone();
                let new_title = format!("{} {}", unread_marker, curr_title);
                let line = Line::styled(new_title, read_style);
                lines.push(line);
            }
            else {
                let curr_title = entries.get(i).expect("Error: Invalid title length").1.clone();
                let new_title = format!("- {}", curr_title);
                let line = Line::styled(new_title, read_style);
                lines.push(line);
            }

        }

        ItemList::new(&lines)
            .title(Some(format!("Entries ({}/{})", unread_len, list_len)))
            .render(area, buf, &mut self.list_state.clone());

    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        None
    }

}

impl Entries {
    pub fn new(selected_feed: Option<&Feed>) -> Self {
        match selected_feed {
            Some(feed) => {
                Self {
                    list_state: ListState::default(),
                    entry_items: get_entries(feed).unwrap_or(vec![]),
                }
            }
            None => {
                Self {
                    list_state: ListState::default(),
                    entry_items: vec![],
                }
            }
        }
    }
}
