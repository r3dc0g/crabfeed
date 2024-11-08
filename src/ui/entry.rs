use super::components::*;
use super::util::{parse_hex, parse_html};
use super::{UiCallback, View};
use crate::config::Settings;
use crate::prelude::EntryData;
use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{ListState, Paragraph, Wrap};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Section {
    Content,
    Links,
}

pub struct Entry {
    entry: Option<EntryData>,
    line_index: u16,
    description: Option<Paragraph<'static>>,
    selected_section: Option<Section>,
    hovered_section: Option<Section>,
    link_state: ListState,
}

impl Entry {
    pub fn new(entry: Option<EntryData>) -> Self {
        Self {
            entry,
            line_index: 0,
            description: None,
            selected_section: None,
            hovered_section: Some(Section::Content),
            link_state: ListState::default(),
        }
    }

    pub fn set_entry(&mut self, entry: EntryData) {
        self.entry = Some(entry.clone());
        self.line_index = 0;
        if let Ok(description) = parse_html(entry.description) {
            self.description = Some(description);
        }
    }
}

impl View for Entry {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings) {
        let primary = parse_hex(&config.colors.primary);
        let secondary = parse_hex(&config.colors.secondary);

        let hovered_style = Style::default().fg(secondary);
        let selected_style = Style::default().fg(primary);

        let content_style = if let Some(section) = &self.selected_section {
            if *section == Section::Content {
                selected_style
            } else {
                Style::default()
            }
        } else {
            if let Some(section) = &self.hovered_section {
                if *section == Section::Content {
                    hovered_style
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            }
        };

        let link_style = if let Some(section) = &self.selected_section {
            if *section == Section::Links {
                selected_style
            } else {
                Style::default()
            }
        } else {
            if let Some(section) = &self.hovered_section {
                if *section == Section::Links {
                    hovered_style
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            }
        };

        let entry_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(3),
                Constraint::Max(80),
                Constraint::Length(10),
            ],
        )
        .split(area);

        let Some(entry) = &self.entry else {
            BlockLabel::new()
                .label(String::from("No Entry Found"))
                .render(entry_layout[0], buf);

            BlockText::default()
                .title(None)
                .paragraph(Paragraph::new("No description".to_string()))
                .style(content_style)
                .margin(Margin::new(
                    (0.05 * entry_layout[1].width as f32) as u16,
                    (0.05 * entry_layout[1].height as f32) as u16,
                ))
                .render(entry_layout[1], buf);

            return;
        };

        BlockLabel::new()
            .label(entry.title.clone())
            .render(entry_layout[0], buf);

        match &self.description {
            Some(description) => {
                BlockText::default()
                    .title(None)
                    .paragraph(description.clone().scroll((self.line_index, 0)))
                    .style(content_style)
                    .margin(Margin::new(
                        (0.05 * entry_layout[1].width as f32) as u16,
                        (0.05 * entry_layout[1].height as f32) as u16,
                    ))
                    .render(entry_layout[1], buf);
            }
            None => {
                BlockText::default()
                    .title(None)
                    .paragraph(Paragraph::new("No Summary".to_string()).wrap(Wrap::default()))
                    .style(content_style)
                    .margin(Margin::new(
                        (0.05 * entry_layout[1].width as f32) as u16,
                        (0.05 * entry_layout[1].height as f32) as u16,
                    ))
                    .render(entry_layout[1], buf);
            }
        }

        let links: Vec<String> = entry
            .links
            .clone()
            .into_iter()
            .map(|link| link.href)
            .collect();

        ItemList::new(&links)
            .title(Some(format!(
                "Links ({}/{})",
                self.link_state.selected().unwrap_or(0) + 1,
                entry.links.len()
            )))
            .style(link_style)
            .render(entry_layout[2], buf, &mut self.link_state.clone());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            KeyCode::Char('y') => {
                let Some(entry) = &self.entry else {
                    return None;
                };
                if let Some(section) = &self.selected_section {
                    match section {
                        Section::Content => {
                            return None;
                        }
                        Section::Links => {
                            if let Some(index) = self.link_state.selected() {
                                let link = &entry.links[index];
                                let mut clipboard: ClipboardContext =
                                    ClipboardProvider::new().unwrap();
                                if let Err(_) = clipboard.set_contents(link.href.clone()) {
                                    // TODO: Add error handling
                                }
                            }
                            return None;
                        }
                    }
                }
                return None;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let Some(entry) = &self.entry else {
                    return None;
                };
                if self.selected_section.is_none() {
                    if let Some(section) = &self.hovered_section {
                        match section {
                            Section::Content => {
                                self.hovered_section = Some(Section::Links);
                                return None;
                            }
                            Section::Links => {
                                self.hovered_section = Some(Section::Content);
                                return None;
                            }
                        }
                    }
                    return None;
                } else if let Some(section) = &self.selected_section {
                    match section {
                        Section::Content => {
                            self.line_index += 1;
                            return None;
                        }
                        Section::Links => {
                            if let Some(index) = self.link_state.selected() {
                                if index + 1 == entry.links.len() {
                                    self.link_state.select(Some(0));
                                } else {
                                    self.link_state.select(Some(index + 1));
                                }
                            } else {
                                self.link_state.select(Some(0));
                            }
                            return None;
                        }
                    }
                } else {
                    return None;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let Some(entry) = &self.entry else {
                    return None;
                };
                if self.selected_section.is_none() {
                    if let Some(section) = &self.hovered_section {
                        match section {
                            Section::Content => {
                                self.hovered_section = Some(Section::Links);
                                return None;
                            }
                            Section::Links => {
                                self.hovered_section = Some(Section::Content);
                                return None;
                            }
                        }
                    }
                } else if let Some(section) = &self.selected_section {
                    match section {
                        Section::Content => {
                            if self.line_index > 0 {
                                self.line_index -= 1;
                            }
                            return None;
                        }
                        Section::Links => {
                            if let Some(index) = self.link_state.selected() {
                                if index == 0 {
                                    self.link_state.select(Some(entry.links.len() - 1));
                                } else {
                                    self.link_state.select(Some(index - 1));
                                }
                            } else {
                                self.link_state.select(Some(entry.links.len() - 1));
                            }
                            return None;
                        }
                    }
                }
                return None;
            }
            KeyCode::Enter => {
                if let Some(_) = &self.selected_section {
                    return None;
                } else {
                    if let Some(section) = &self.hovered_section {
                        match section {
                            Section::Content => {
                                self.selected_section = Some(Section::Content);
                                self.hovered_section = None;
                                return None;
                            }
                            Section::Links => {
                                self.selected_section = Some(Section::Links);
                                self.hovered_section = None;
                                return None;
                            }
                        }
                    }
                    return None;
                }
            }
            KeyCode::Char('h') | KeyCode::Char('q') | KeyCode::Esc => {
                let Some(entry) = &self.entry else {
                    return Some(Box::new(move |app| {
                        app.ui.back();
                        Ok(())
                    }));
                };
                self.link_state.select(None);

                if key.code != KeyCode::Char('h') {
                    if let Some(section) = &self.selected_section {
                        match section {
                            Section::Content => {
                                self.hovered_section = Some(Section::Content);
                            }
                            Section::Links => {
                                self.hovered_section = Some(Section::Links);
                            }
                        }
                        self.selected_section = None;
                        return None;
                    }
                } else {
                    self.hovered_section = Some(Section::Content);
                    self.selected_section = None;
                }

                return Some(Box::new(move |app| {
                    app.ui.back();
                    Ok(())
                }));
            }
            _ => None,
        }
    }
}
