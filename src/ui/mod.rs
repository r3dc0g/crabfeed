use std::io::stdout;
use crate::prelude::*;
use crate::error::Error;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::db::{get_entries, get_feeds, select_feed};

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

pub fn start_ui() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(render_start_page)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
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

fn render_start_page(frame: &mut Frame) {

    let feeds = get_feeds().expect("Cannot connect to database");

    let mut feed_titles = Vec::new();

    for feed in feeds {

        let Some(title) = feed.title else {
            continue;
        };

        feed_titles.push(title);
    }

    let mut feed_list = FeedList::new();
    feed_list.set_items(feed_titles);
    feed_list.state.select(Some(0));


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

    frame.render_widget(
        Paragraph::new("Crabfeed")
            .block(Block::default()
            .title("Greeting")
            .borders(Borders::ALL)),
        main_layout[0],
    );

    render_feeds(frame, &mut feed_list, info_layout[0]);

    let curr_feed = select_feed(feed_list.items.iter().nth(0).expect("Error reading feed")).expect("Error finding feed");

    render_entries(frame, curr_feed, info_layout[1]);

}

fn render_feeds(frame: &mut Frame, feeds: &mut FeedList, area: Rect) {

    let items: Vec<ListItem> = feeds
        .items
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();

    let list = List::new(items);

    frame.render_stateful_widget(
        list.block(
            Block::default()
            .title("Feeds")
            .borders(Borders::ALL)
        ),
        area,
        &mut feeds.state
    );
}

fn render_entries(frame: &mut Frame, feed: Feed, area: Rect) {

    let entries = get_entries(&feed).expect("Cannot connect to database");

    let mut entry_titles = Vec::new();

    for entry in entries {

        let Some(title) = entry.title else {
            continue;
        };

        entry_titles.push(title);
    }

    let mut entry_list = EntryList::new();
    entry_list.set_items(entry_titles);
    entry_list.state.select(Some(0));


    let items: Vec<ListItem> = entry_list
        .items
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();

    let list = List::new(items);

    frame.render_stateful_widget(
        list.block(
            Block::default()
            .title("Entries")
            .borders(Borders::ALL)
        ),
        area,
        &mut entry_list.state
    );
}
