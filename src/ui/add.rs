use crate::{config::Settings, data::data::DataEvent};

use super::{
    components::{BlockText, Popup},
    UiCallback, View,
};

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

pub struct Add {
    pub input: Vec<char>,
    pub input_cursor_position: usize,
    pub input_i: usize,
}

impl Add {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            input_cursor_position: 0,
            input_i: 0,
        }
    }

    fn reset(&mut self) {
        self.input = Vec::new();
        self.input_cursor_position = 0;
        self.input_i = 0;
    }
}

impl View for Add {
    fn render(&self, area: Rect, buf: &mut Buffer, _config: &Settings) {
        Popup::new(Some(
            BlockText::default()
                .title(Some("Feed URL".to_string()))
                .paragraph(
                    Paragraph::new(
                        Line::from(vec![
                            Span::from(&self.input.iter().collect::<String>().replace("\n", ""))
                                .style(Style::default().underlined()),
                            Span::raw("█"),
                        ])
                        .alignment(Alignment::Left),
                    )
                    .wrap(Wrap::default()),
                ),
        ))
        .height(3)
        .width(60)
        .render(area, buf);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            _ if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc => {
                Some(Box::new(move |app| {
                    app.ui.unset_popup();
                    Ok(())
                }))
            }
            _ if key.code == KeyCode::Char('v') && key.modifiers == KeyModifiers::CONTROL => {
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                if let Ok(mut contents) = clipboard.get_contents() {
                    contents.retain(|c| c != '\n');
                    for c in contents.chars() {
                        self.input.insert(self.input_i, c);
                        self.input_i += 1;
                        self.input_cursor_position += 1;
                    }
                }

                return None;
            }
            KeyCode::Char(c) => {
                self.input.insert(self.input_i, c);
                self.input_i += 1;
                self.input_cursor_position += 1;

                return None;
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() && self.input_i > 0 {
                    self.input.remove(self.input_i - 1);
                    self.input_i -= 1;
                    self.input_cursor_position -= 1;
                }

                return None;
            }
            KeyCode::Delete => {
                if !self.input.is_empty() && self.input_i < self.input.len() {
                    self.input.remove(self.input_i);
                }

                return None;
            }
            KeyCode::Enter => {
                if self.input.is_empty() {
                    self.reset();
                    return Some(Box::new(move |app| {
                        app.ui.back();
                        Ok(())
                    }));
                }

                let url = self.input.iter().collect::<String>().replace("\n", "");
                self.reset();
                Some(Box::new(move |app| {
                    app.dispatch(DataEvent::AddFeed(url.clone()))?;
                    app.dispatch(DataEvent::Refresh)?;
                    app.ui.back();
                    Ok(())
                }))
            }
            _ => None,
        }
    }
}
