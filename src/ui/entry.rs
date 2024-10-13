use super::components::*;
use super::util::{parse_hex, parse_html};
use super::{UiCallback, View};
use crate::config::Settings;
use crate::db::{find_entry_links, find_media_links, select_content, select_media};
use crate::prelude::{Entry as EntryModel, Link};
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
    entry: Option<EntryModel>,
    line_index: u16,
    link_items: Vec<Link>,
    content: Option<Paragraph<'static>>,
    summary: Option<Paragraph<'static>>,
    description: Option<Paragraph<'static>>,
    selected_section: Option<Section>,
    hovered_section: Option<Section>,
    link_state: ListState,
}

impl Entry {
    pub fn new(entry: Option<EntryModel>) -> Self {
        Self {
            entry,
            line_index: 0,
            link_items: Vec::new(),
            content: None,
            summary: None,
            description: None,
            selected_section: None,
            hovered_section: Some(Section::Content),
            link_state: ListState::default(),
        }
    }

    pub fn set_entry(&mut self, entry: Option<EntryModel>) {
        self.entry = entry;
        self.line_index = 0;
        self.link_items = Vec::new();
        self.get_content();
        self.get_summary();
        self.get_description();
        self.get_links();
    }

    fn get_content(&mut self) {
        if let Some(entry) = &self.entry {
            if let Some(content_id) = entry.content_id {
                if let Ok(content) = select_content(&content_id) {
                    if let Some(body) = content.body {
                        if let Ok(tui_content) = parse_html(body.clone()) {
                            self.content = Some(tui_content);
                        } else {
                            self.content = Some(Paragraph::new(body.clone()).wrap(Wrap::default()));
                        }
                    }
                }
            }
        }
    }

    fn get_summary(&mut self) {
        if let Some(entry) = &self.entry {
            if let Some(summary) = &entry.summary {
                if let Ok(tui_summary) = parse_html(summary.clone()) {
                    self.summary = Some(tui_summary);
                } else {
                    self.summary = Some(Paragraph::new(summary.clone()).wrap(Wrap::default()));
                }
            }
        }
    }

    fn get_description(&mut self) {
        if let Some(entry) = &self.entry {
            if let Some(media_id) = entry.media_id {
                if let Ok(media) = select_media(&media_id) {
                    if let Some(description) = &media.description {
                        if let Ok(tui_description) = parse_html(description.clone()) {
                            self.description = Some(tui_description);
                        } else {
                            self.description =
                                Some(Paragraph::new(description.clone()).wrap(Wrap::default()));
                        }
                    }
                }
            }
        }
    }

    fn get_links(&mut self) {
        if let Some(entry) = &self.entry {
            if let Ok(links) = find_entry_links(entry.id) {
                for link in links.iter() {
                    self.link_items.push(link.clone());
                }
            }

            if let Some(media_id) = entry.media_id {
                if let Ok(links) = find_media_links(media_id) {
                    for link in links.iter() {
                        self.link_items.push(link.clone());
                    }
                }
            }
        }
    }
}

impl View for Entry {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings) {
        let primary = parse_hex(&config.colors.primary);
        let secondary = parse_hex(&config.colors.secondary);

        let hovered_style = Style::default().fg(secondary);
        let selected_style = Style::default().fg(primary);

        let possible_entry = &self.entry;

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

        match possible_entry {
            Some(entry) => {
                let entry_layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Length(3),
                        Constraint::Max(80),
                        Constraint::Length(10),
                    ],
                )
                .split(area);

                BlockLabel::new()
                    .label(entry.title.clone().unwrap_or("No Title".to_string()))
                    .render(entry_layout[0], buf);

                match &self.content {
                    Some(content) => {
                        BlockText::default()
                            .title(None)
                            .paragraph(content.clone().scroll((self.line_index, 0)))
                            .style(content_style)
                            .margin(Margin::new(
                                (0.05 * entry_layout[1].width as f32) as u16,
                                (0.05 * entry_layout[1].height as f32) as u16,
                            ))
                            .render(entry_layout[1], buf);
                    }
                    None => match &self.summary {
                        Some(summary) => {
                            BlockText::default()
                                .title(None)
                                .paragraph(summary.clone().scroll((self.line_index, 0)))
                                .style(content_style)
                                .margin(Margin::new(
                                    (0.05 * entry_layout[1].width as f32) as u16,
                                    (0.05 * entry_layout[1].height as f32) as u16,
                                ))
                                .render(entry_layout[1], buf);
                        }
                        None => match &self.description {
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
                                    .paragraph(
                                        Paragraph::new("No Summary".to_string())
                                            .wrap(Wrap::default()),
                                    )
                                    .style(content_style)
                                    .margin(Margin::new(
                                        (0.05 * entry_layout[1].width as f32) as u16,
                                        (0.05 * entry_layout[1].height as f32) as u16,
                                    ))
                                    .render(entry_layout[1], buf);
                            }
                        },
                    },
                }

                let links: Vec<String> = self
                    .link_items
                    .clone()
                    .into_iter()
                    .map(|link| link.href)
                    .collect();

                ItemList::new(&links)
                    .title(Some(format!(
                        "Links ({}/{})",
                        self.link_state.selected().unwrap_or(0) + 1,
                        self.link_items.len()
                    )))
                    .style(link_style)
                    .render(entry_layout[2], buf, &mut self.link_state.clone());
            }

            None => {
                BlockText::default()
                    .title(None)
                    .paragraph(Paragraph::new("Error: No Entry Found".to_string()))
                    .style(content_style)
                    .render(area, buf);
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            KeyCode::Char('y') => {
                if let Some(section) = &self.selected_section {
                    match section {
                        Section::Content => {
                            return None;
                        }
                        Section::Links => {
                            if let Some(index) = self.link_state.selected() {
                                let link = &self.link_items[index];
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
                                if index + 1 == self.link_items.len() {
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
                                    self.link_state.select(Some(self.link_items.len() - 1));
                                } else {
                                    self.link_state.select(Some(index - 1));
                                }
                            } else {
                                self.link_state.select(Some(self.link_items.len() - 1));
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
                    app.ui.update_entries();
                    app.ui.back();
                    Ok(())
                }));
            }
            _ => None,
        }
    }
}
