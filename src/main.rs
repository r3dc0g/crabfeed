mod db;
mod schema;
mod prelude;
mod error;
mod ui;
mod app;
mod network;
mod event;
mod handlers;

use std::{
    io::stdout, ops::Deref, sync::Arc
};

use error::Error;

use crossterm::{
    execute, terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        SetTitle
    }, ExecutableCommand
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal
};

use tokio::sync::Mutex;
use crate::app::App;
use crate::network::{IOEvent, Network};
// use crate::handlers::handle_app;

pub type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IOEvent>();

    // Initialise app stat
    let app = Arc::new(Mutex::new(App::new(
      sync_io_tx,
    )));

    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
      let mut network = Network::new(&app);
      start_tokio(sync_io_rx, &mut network).unwrap();
    });

    start_ui(&cloned_app).await?;

    Ok(())
}

#[tokio::main]
async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IOEvent>, network: &mut Network) -> Result<()> {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_io_event(io_event).await?;
    }

    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let mut stdout = stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);

    backend.execute(SetTitle("crabfeed"))?;

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let events = event::Events::new(250);

    let mut is_first_render = true;

    loop {

        // In the loop the order goes
        // lock app state
        // get app size on first render
        // get the current route
        // match events
        // Get data for first render

        let mut app = app.lock().await;

        if is_first_render {
            app.update_feed_items();
        }

        if let Ok(size) = terminal.backend().size() {
            if is_first_render || app.size != size {
                app.size = size;
            }
        }

        // let current_route = app.get_current_route();
        terminal.draw(|f| {
            f.render_widget(
                app.deref(),
                app.size
            );
        })?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == event::Key::Char('q') {
                    break;
                }

                handlers::handle_app(key, &mut app);
            }
            _ => {}
        }

        is_first_render = false;
    }

    terminal.show_cursor()?;
    close_app()?;
    Ok(())
}

fn close_app() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

// #[cfg(test)]
// mod tests {

//     use crate::error::Error;
//     use std::fs::read_to_string;

//     pub type Result<T> = core::result::Result<T, Error>;

//     #[tokio::test]
//     async fn test_feed_insertion() -> Result<()> {
//         use crate::control::get_feed;
//         use crate::db::*;

//         let mut lines = Vec::new();

//         for line in read_to_string("urls")?.lines() {
//             lines.push(line.to_string());
//         }

//         let conn = &mut connect()?;

//         for line in lines {
//             match get_feed(line).await {
//                 Ok(test_feed) => {
//                     insert_feed(conn, test_feed)?;
//                 },
//                 Err(e) => {
//                     println!("{:?}", e);
//                     ()
//                 }
//             }

//         }

//         Ok(())
//     }
// }
