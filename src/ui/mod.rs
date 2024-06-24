use crate::db::select_content;
use crate::app::{ActiveBlock, App, RouteId};
use ratatui::prelude::*;

use ratatui::widgets::Wrap;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListState, Paragraph},
    style::{Style, Color},
    Frame,
};

#[derive(Default)]
struct ItemList {
    items: Vec<String>,
    state: ListState,
}

impl ItemList {
    pub fn new() -> ItemList {
       ItemList::default()
    }

    pub fn set_items(&mut self, items: Vec<String>) {

        self.items = items;

        self.state = ListState::default();
    }

}

pub fn render_start_page(frame: &mut Frame, app: &App) {

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Max(100),
            Constraint::Length(3),
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
            Line::from("Crabfeed")
            .alignment(Alignment::Center)
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
        ),
        main_layout[0],
    );

    match app.get_current_route().id {
        RouteId::Home => {
            let info_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(main_layout[1]);

            render_feeds(frame, app, info_layout[0]);


            render_entries(frame, app, info_layout[1]);
        }
        RouteId::Entry => {
            render_entry(frame, app, main_layout[1]);
        }
    }

    if app.is_loading {
        frame.render_widget(
            Paragraph::new(
                Line::from("loading...")
                .alignment(Alignment::Center)
            )
            .block(
                Block::default()
                .borders(Borders::ALL)
            ),
            main_layout[2],
        );
    } else {

        if app.get_current_route().active_block == ActiveBlock::Input {
            render_add_feed(frame, app, main_layout[2])
        }
        else {
            if let Some(error) = &app.error_msg {
                frame.render_widget(
                    Paragraph::new(
                        Line::from(error.as_str())
                        .alignment(Alignment::Center)
                    )
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                    ),
                    main_layout[2],
                );

            }
            else {
                frame.render_widget(
                    Paragraph::new(
                        Line::from("Ctrl+a to add feed, Ctrl+d to delete feed, (ESC/Q) to quit")
                        .alignment(Alignment::Center)
                    )
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                    ),
                    main_layout[2],
                );
            }

        }

    }

}

fn render_feeds(frame: &mut Frame, app: &App, area: Rect) {

    // Very fun code that separtes the feed item tuples into two vectors
    let (titles, _): (Vec<String>, Vec<i32>) = app.feed_items.clone().into_iter().map(|(title, id)| (title, id)).unzip();

    let mut feed_list = ItemList::new();
    feed_list.set_items(titles);
    feed_list.state.select(app.selected_feed_index);

    let list = List::new(feed_list.items.clone());

    let mut style = Style::default();

    if app.get_current_route().active_block == ActiveBlock::Feeds {
        style = style.fg(Color::Red);
    }

    frame.render_stateful_widget(
        list.block(
            Block::default()
            .title("Feeds")
            .borders(Borders::ALL)
            .border_style(style)
        )
        .highlight_style(
                Style::default()
                    .bg(Color::Red)
        ),
        area,
        &mut feed_list.state,
    );
}

fn render_entries(frame: &mut Frame, app: &App, area: Rect) {

    let (titles, entries): (Vec<String>, (Vec<i32>, Vec<bool>)) = app.entry_items.clone()
                                                                                 .into_iter()
                                                                                 .map(|(title, entry)| (title, entry))
                                                                                 .unzip();

    let (_, read): (Vec<i32>, Vec<bool>) = entries;

    // This is just a state item and should be integrated into the app struct
    // This would prevent the need to clone the titles vector
    let mut entry_list = ItemList::new();
    entry_list.set_items(titles.clone());
    entry_list.state.select(app.selected_entry_index);

    let mut lines = vec![];
    let mut read_style = Style::default();

    let list_len = titles.len();

    for i in 0..list_len {

        if *read.get(i).expect("Error: More titles than entry items") {
            read_style = read_style.bold();
        }
        let line = Line::styled(titles.get(i).expect("Error: Invalid title length"), read_style);
        lines.push(line);
    }

    let list = List::new(lines);

    let mut style = Style::default();

    if app.get_current_route().active_block == ActiveBlock::Entries {
        style = style.fg(Color::Red);
    }

    frame.render_stateful_widget(
        list.block(
            Block::default()
            .title("Entries")
            .borders(Borders::ALL)
            .border_style(style)
        )
        .highlight_style(
                Style::default()
                    .bg(Color::Red)
        ),
        area,
        &mut entry_list.state
    );
}

fn render_entry(frame: &mut Frame, app: &App, area: Rect) {

    let entry_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ],
    )
    .split(area);

    let possible_entry = &app.entry;

    match possible_entry {

        Some(entry) => {
            let (links, _): (Vec<String>, Vec<i32>) = app.link_items.clone().into_iter().map(|(link, id)| (link, id)).unzip();

            let link_list = List::new(links.clone());

            let summary = entry.summary.clone().unwrap_or("No Summary".to_string());

            frame.render_widget(
                Paragraph::new(entry.title.clone().unwrap_or("No Title".to_string()))
                    .block(Block::default()
                        .borders(Borders::ALL)
                    ),
                entry_layout[0],
            );

            match select_content(&entry.content_id.unwrap_or(-1)) {
                Ok(content) => {
                    frame.render_widget(
                        Paragraph::new(content.body.unwrap_or("No Content".to_string()))
                            .wrap(Wrap::default())
                            .block(Block::default()
                                .borders(Borders::ALL)
                            ),
                        entry_layout[1],
                    );
                }
                Err(_) => {
                    frame.render_widget(
                        Paragraph::new(summary)
                            .wrap(Wrap::default())
                            .block(Block::default()
                                .borders(Borders::ALL)
                            ),
                        entry_layout[1],
                    );
                }
            }

            frame.render_widget(
                link_list.block(Block::default()
                    .title("Links")
                    .borders(Borders::ALL)),
                entry_layout[2],
            );
        }

        None => {
            frame.render_widget(
                Paragraph::new(
                    Span::raw("Error: No Entry Found")
                )
                .block(Block::default()
                    .borders(Borders::ALL)
                ),
                area
            );
        }
    }
}

fn render_add_feed(frame: &mut Frame, app: &App, area: Rect) {

    let mut style = Style::default();

    if app.get_current_route().active_block == ActiveBlock::Input {
        style = style.fg(Color::Red);
    }

    frame.render_widget(
        Paragraph::new(
            Line::from(
                vec![
                    Span::raw("URL: ").style(Style::default().bold()),
                    Span::from(
                        &app.input.iter().collect::<String>()
                    ).style(Style::default().underlined())
                ]
            )
            .alignment(Alignment::Left)
        )
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(style)
        ),
        area,
    );
}
