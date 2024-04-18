use crate::error::Error;
use crate::app::App;
use ratatui::prelude::*;

use crossterm::event::{self, Event, KeyCode};

// use crate::db::{get_entries, get_feeds, select_feed};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
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

    // frame.render_widget(
    //     Paragraph::new("Feeds")
    //         .block(Block::default()
    //         .title("Feeds")
    //         .borders(Borders::ALL)),
    //     info_layout[0]
    // );

    frame.render_widget(
        Paragraph::new("Entries")
            .block(Block::default()
            .title("Entries")
            .borders(Borders::ALL)),
        info_layout[1]
    );

    // render_entries(frame, app, info_layout[1]);

}

fn render_feeds(frame: &mut Frame, app: &App, area: Rect) {

    let mut items: Vec<String> = app.feeds
        .iter()
        .map(|f| f.title.clone().unwrap_or("No Title".to_string()))
        .collect();

    if items.is_empty() {
        items.push("No feeds available".to_string());
    }

    let mut feed_list = FeedList::new();
    feed_list.set_items(items);
    feed_list.state.select(Some(0));

    let list = List::new(feed_list.items.clone());

    frame.render_stateful_widget(
        list.block(
            Block::default()
            .title("Feeds")
            .borders(Borders::ALL)
        ),
        area,
        &mut feed_list.state,
    );
}

// fn render_entries(frame: &mut Frame, app: &App, area: Rect) {

//     let feed_indx = app.selected_feed_index.unwrap_or(0);
//     let feed = app.feeds.get(feed_indx);

//     let entries = get_entries(&feed).expect("Cannot connect to database");

//     let mut entry_titles = Vec::new();

//     for entry in entries {

//         let Some(title) = entry.title else {
//             continue;
//         };

//         entry_titles.push(title);
//     }

//     let mut entry_list = EntryList::new();
//     entry_list.set_items(entry_titles);
//     entry_list.state.select(Some(0));


//     let items: Vec<ListItem> = entry_list
//         .items
//         .iter()
//         .map(|i| ListItem::new(i.as_str()))
//         .collect();

//     let list = List::new(items);

//     frame.render_stateful_widget(
//         list.block(
//             Block::default()
//             .title("Entries")
//             .borders(Borders::ALL)
//         ),
//         area,
//         &mut entry_list.state
//     );
// }
