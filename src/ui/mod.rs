mod components;

use crate::app::{ActiveBlock, App, RouteId};
use components::*;

use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{ListState, Paragraph},
    style::{Style, Color}
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let selected_style = Style::default().fg(Color::Red);
        let unselected_style = Style::default();

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

        match self.get_current_route().id {
            RouteId::Home => {

                let (feed_titles, _): (Vec<String>, Vec<i32>) = self.feed_items.clone().into_iter().map(|(title, id)| (title, id)).unzip();

                let (entry_titles, entries): (Vec<String>, (Vec<i32>, Vec<bool>)) = self.entry_items.clone().into_iter().map(|(title, entry)| (title, entry)).unzip();
                let (_, read): (Vec<i32>, Vec<bool>) = entries;
                let list_len  = entry_titles.len();
                let mut unread_len = 0;
                let unread_marker = "*";
                let mut lines = vec![];

                for i in 0..list_len {

                    let mut read_style = Style::default();

                    let has_read = read.get(i).expect("Error: More read items than entry items");

                    if !*has_read {
                        read_style = read_style.bold();
                        unread_len += 1;
                        let curr_title = entry_titles.get(i).expect("Error: Invalid title length");
                        let new_title = format!("{} {}", unread_marker, curr_title);
                        let line = Line::styled(new_title, read_style);
                        lines.push(line);
                    }
                    else {
                        let curr_title = entry_titles.get(i).expect("Error: Invalid title length");
                        let new_title = format!("- {}", curr_title);
                        let line = Line::styled(new_title, read_style);
                        lines.push(line);
                    }

                }



                let lists_section = Layout::new(
                    Direction::Horizontal,
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                )
                .split(app_layout[1]);

                ItemList::new(&feed_titles)
                    .title(Some("Feeds".to_string()))
                    .style(
                        match self.get_current_route().active_block {
                            ActiveBlock::Feeds => selected_style,
                            _ => unselected_style
                        }
                    )
                    .render(lists_section[0], buf, &mut self.feed_list_state.clone());

                ItemList::new(&lines)
                    .title(Some(format!("Entries ({}/{})", unread_len, self.total_entries)))
                    .style(
                        match self.get_current_route().active_block {
                            ActiveBlock::Entries => selected_style,
                            _ => unselected_style
                        }
                    )
                    .render(lists_section[1], buf, &mut self.entry_list_state.clone());

                if self.get_current_route().active_block == ActiveBlock::Input {
                    Popup::new(
                        Some(
                            BlockText::default()
                                .title(Some("Add Feed".to_string()))
                                .paragraph(
                                    Paragraph::new(
                                        Line::from(
                                            vec![
                                                Span::raw("FEED URL: ").style(Style::default().bold()),
                                                Span::from(
                                                    &self.input.iter().collect::<String>().replace("\n", "")
                                                ).style(Style::default().underlined()),
                                                Span::raw("_"),
                                            ]
                                        )
                                        .alignment(Alignment::Left)
                                    )
                                    .wrap(Wrap::default())
                                )
                        )
                    )
                    .height(8)
                    .width(60)
                    .render(app_layout[1], buf);
                }

            }

            RouteId::Entry => {

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
                        .split(app_layout[1]);

                        BlockLabel::new()
                            .label(
                                entry.title.clone()
                                    .unwrap_or("No Title".to_string())
                            )
                            .render(entry_layout[0], buf);

                        match &self.entry_content {
                            Some(content) => {

                                BlockText::default()
                                    .title(None)
                                    .paragraph(
                                        content.clone().scroll((self.entry_line_index, 0))
                                    )
                                    .margin(
                                        Margin::new(
                                            (0.05 * entry_layout[1].width as f32) as u16,
                                            (0.03 * entry_layout[1].height as f32) as u16
                                        )
                                    )
                                    .render(entry_layout[1], buf);
                            }
                            None => {
                                match &self.entry_summary {
                                    Some(summary) => {
                                        BlockText::default()
                                            .title(None)
                                            .paragraph(
                                                summary.clone().scroll((self.entry_line_index, 0))
                                            )
                                            .margin(
                                                Margin::new(
                                                    (0.05 * entry_layout[1].width as f32) as u16,
                                                    (0.03 * entry_layout[1].height as f32) as u16
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
                                                    (0.03 * entry_layout[1].height as f32) as u16
                                                )
                                            )
                                            .render(entry_layout[1], buf);
                                    }
                                }
                            }
                        }

                        let (links, _): (Vec<String>, Vec<i32>) = self.link_items.clone().into_iter().map(|(link, id)| (link, id)).unzip();

                        ItemList::new(&links)
                            .title(Some("Links".to_string()))
                            .style(unselected_style)
                            .render(entry_layout[2], buf, &mut ListState::default());

                    }

                    None => {
                        BlockText::default()
                            .title(None)
                            .paragraph(
                                Paragraph::new("Error: No Entry Found".to_string())
                            )
                            .render(app_layout[1], buf);
                    }
                }
            }
        }

        BlockLabel::new()
            .label("Ctrl+a to add feed, Ctrl+d to delete feed, (ESC/Q) to quit".to_string())
            .render(app_layout[2], buf);

    }
}
