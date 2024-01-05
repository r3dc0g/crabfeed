use std::io::stdout;
use crate::error::Error;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::db::get_feed;

pub type Result<T> = core::result::Result<T, Error>;

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
        }
    }
    Ok(false)
}

fn render_start_page(frame: &mut Frame) {

    let feeds = get_feed().expect("Cannot connect to database");

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


    let feeds_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10); 10])
        .margin(2)
        .split(info_layout[0]);

    frame.render_widget(
        Block::default()
        .title("Feeds")
        .borders(Borders::ALL),
        info_layout[0]
    );

    frame.render_widget(
        Block::default()
        .title("Entries")
        .borders(Borders::ALL),
        info_layout[1]
    );

    for space in 0..9 {

        let Some(title) = feeds[space].title.clone() else {
            frame.render_widget(
                Paragraph::new("Untitled"),
                feeds_layout[space],
            );

            continue;
        };

        frame.render_widget(
            Paragraph::new(title)
            .block(Block::default()
                   .padding(Padding::uniform(0))),
            feeds_layout[space],
        );
    }
}
