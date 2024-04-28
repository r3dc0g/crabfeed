use crate::{db::select_entries, error::Error};
use crate::app::{ActiveBlock, App};
use ratatui::prelude::*;

use crossterm::event::{self, Event, KeyCode};

// use crate::db::{get_entries, get_feeds, select_feed};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    style::{Style, Color},
    Frame,
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Default)]
struct FeedList {
    items: Vec<String>,
    state: ListState,
}

impl FeedList {
    pub fn new() -> FeedList {
       FeedList::default()
    }

    pub fn set_items(&mut self, items: Vec<String>) {

        self.items = items;

        self.state = ListState::default();
    }

}

#[derive(Default)]
struct EntryList {
    items: Vec<String>,
    state: ListState,
}

impl EntryList {
    pub fn new() -> EntryList {
       EntryList::default()
    }

    pub fn set_items(&mut self, items: Vec<String>) {
       self.items = items;

       self.state = ListState::default();
    }

}


fn handle_events() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }

            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('j') {

            }
        }
    }
    Ok(false)
}

pub fn render_start_page(frame: &mut Frame, app: &App) {

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
             Constraint::Percentage(10),
             Constraint::Percentage(90),
        ])
        .split(frame.size());

    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_layout[1]);

    if app.is_loading {
        frame.render_widget(
            Paragraph::new("Crabfeed - loading...")
                .block(Block::default()
                .title("Greeting")
                .borders(Borders::ALL)),
            main_layout[0],
        );
    }
    else {
        frame.render_widget(
            Paragraph::new("Crabfeed")
                .block(Block::default()
                .title("Greeting")
                .borders(Borders::ALL)),
            main_layout[0],
        );

    }


    render_feeds(frame, app, info_layout[0]);


    render_entries(frame, app, info_layout[1]);

}

fn render_feeds(frame: &mut Frame, app: &App, area: Rect) {

    // Very fun code that separtes the feed item tuples into two vectors
    let (titles, _): (Vec<String>, Vec<i32>) = app.feed_items.clone().into_iter().map(|(title, id)| (title, id)).unzip();

    let mut feed_list = FeedList::new();
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

    let (titles, _): (Vec<String>, Vec<i32>) = app.entry_items.clone().into_iter().map(|(title, id)| (title, id)).unzip();

    let mut entry_list = FeedList::new();
    entry_list.set_items(titles.clone());
    entry_list.state.select(app.selected_entry_index);

    let list = List::new(entry_list.items.clone());

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
