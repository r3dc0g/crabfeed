use crate::prelude::Entry as EntryModel;
use super::components::*;
use super::{View, UiCallback};
use ratatui::prelude::*;
use ratatui::widgets::{ListState, Paragraph, Wrap};
use crossterm::event::KeyEvent;

pub struct Entry {
    entry: Option<EntryModel>,
    list_state: ListState,
    line_index: u16,
    link_items: Vec<(String, i32)>,
    content: Option<Paragraph<'static>>,
    summary: Option<Paragraph<'static>>,
    description: Option<Paragraph<'static>>,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            entry: None,
            list_state: ListState::default(),
            line_index: 0,
            link_items: Vec::new(),
            content: None,
            summary: None,
            description: None,
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

                let (links, _): (Vec<String>, Vec<i32>) = self.link_items.clone().into_iter().map(|(link, id)| (link, id)).unzip();

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
        None
    }
}

