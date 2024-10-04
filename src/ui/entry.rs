use crate::db::{find_entry_links, find_media_links, select_content, select_media};
use crate::prelude::{Entry as EntryModel, Link};
use super::components::*;
use super::util::parse_html;
use super::{View, UiCallback};
use ratatui::prelude::*;
use ratatui::widgets::{ListState, Paragraph, Wrap};
use crossterm::event::{KeyCode, KeyEvent};

pub struct Entry {
    entry: Option<EntryModel>,
    line_index: u16,
    link_items: Vec<Link>,
    content: Option<Paragraph<'static>>,
    summary: Option<Paragraph<'static>>,
    description: Option<Paragraph<'static>>,
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
        }
    }

    pub fn set_entry(&mut self, entry: Option<EntryModel>) {
        self.entry = entry;
        self.line_index = 0;
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
                        }
                        else {
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
                }
                else {
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
                        }
                        else {
                            self.description = Some(Paragraph::new(description.clone()).wrap(Wrap::default()));
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
    fn render(&self, area: Rect, buf: &mut Buffer) {

        let possible_entry = self.entry.clone();

        match possible_entry {

            Some(entry) => {

                let entry_layout = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Length(3),
                        Constraint::Max(80),
                        Constraint::Length(10),
                    ]
                )
                .split(area);

                BlockLabel::new()
                    .label(
                        entry.title.clone()
                            .unwrap_or("No Title".to_string())
                    )
                    .render(entry_layout[0], buf);

                match &self.content {
                    Some(content) => {

                        BlockText::default()
                            .title(None)
                            .paragraph(
                                content.clone().scroll((self.line_index, 0))
                            )
                            .margin(
                                Margin::new(
                                    (0.05 * entry_layout[1].width as f32) as u16,
                                    (0.05 * entry_layout[1].height as f32) as u16
                                )
                            )
                            .render(entry_layout[1], buf);
                    }
                    None => {
                        match &self.summary {
                            Some(summary) => {
                                BlockText::default()
                                    .title(None)
                                    .paragraph(
                                        summary.clone().scroll((self.line_index, 0))
                                    )
                                    .margin(
                                        Margin::new(
                                            (0.05 * entry_layout[1].width as f32) as u16,
                                            (0.05 * entry_layout[1].height as f32) as u16
                                        )
                                    )
                                    .render(entry_layout[1], buf);
                            }
                            None => {
                                match &self.description {
                                    Some(description) => {
                                        BlockText::default()
                                            .title(None)
                                            .paragraph(
                                                description.clone().scroll((self.line_index, 0))
                                            )
                                            .margin(
                                                Margin::new(
                                                    (0.05 * entry_layout[1].width as f32) as u16,
                                                    (0.05 * entry_layout[1].height as f32) as u16
                                                )
                                            )
                                            .render(entry_layout[1], buf);
                                    }
                                    None => {
                                        BlockText::default()
                                            .title(None)
                                            .paragraph(
                                                Paragraph::new("No Summary".to_string())
                                                    .wrap(Wrap::default())
                                            )
                                            .margin(
                                                Margin::new(
                                                    (0.05 * entry_layout[1].width as f32) as u16,
                                                    (0.05 * entry_layout[1].height as f32) as u16
                                                )
                                            )
                                            .render(entry_layout[1], buf);

                                    }
                                }
                            }
                        }
                    }
                }

                let links: Vec<String> = self.link_items.clone().into_iter().map(|link| link.href).collect();

                ItemList::new(&links)
                    .title(Some("Links".to_string()))
                    .render(entry_layout[2], buf, &mut ListState::default());

            }

            None => {
                BlockText::default()
                    .title(None)
                    .paragraph(
                        Paragraph::new("Error: No Entry Found".to_string())
                    )
                    .render(area, buf);
            }
        }

    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.line_index += 1;
                return None;
            },
            KeyCode::Char('k') | KeyCode::Up => {
                if self.line_index > 0 {
                    self.line_index -= 1;
                }
                return None;
            },
            KeyCode::Char('h') | KeyCode::Esc => {
                return Some(Box::new(
                    move |app| {
                        app.ui.update_entries();
                        app.ui.back();
                        Ok(())
                    }
                ))
            }
            _ => {
                None
            }
        }
    }
}

