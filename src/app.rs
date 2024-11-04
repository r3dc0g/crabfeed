use crate::config::Settings;
use crate::data::data::{DataEvent, DataHandler};
use crate::event::{EventHandler, TerminalEvent};
use crate::time::Tick;
use crate::tui::Tui;
use crate::ui::ui::Ui;
use crate::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::Frame;
use std::io;

#[derive(Clone, Default, PartialEq, Debug)]
pub enum RouteId {
    #[default]
    Home,
    Entry,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ActiveBlock {
    #[default]
    Feeds,
    Entries,
    Entry,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

impl Route {
    pub fn new(id: RouteId, active_block: ActiveBlock) -> Self {
        Self { id, active_block }
    }
}

pub struct App {
    pub is_running: bool,
    pub is_loading: bool,
    pub ui: Ui,
    pub data_handler: DataHandler,
}

impl App {
    pub fn new(config: Settings) -> Self {
        App {
            is_running: true,
            is_loading: false,
            ui: Ui::new(config.clone()),
            data_handler: DataHandler::new(config.database_url.clone()),
        }
    }

    pub fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let event_handler = EventHandler::new();
        let mut tui = Tui::new(backend, event_handler)?;

        self.dispatch(DataEvent::ReloadFeeds)?;

        while self.is_running {
            match tui.event_handler.next()? {
                TerminalEvent::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        self.is_running = false;
                    }
                    self.handle_key_event(key);
                }
                TerminalEvent::Mouse(mouse) => {
                    self.handle_mouse_event(mouse);
                }
                TerminalEvent::Resize(w, h) => {
                    tui.resize(Rect::new(0, 0, w, h))?;
                }
                TerminalEvent::Tick { tick } => {
                    self.handle_tick_event(tick);
                    tui.draw(&mut self.ui)?;
                }
            }
        }
        self.data_handler.abort();
        tui.exit()?;
        Ok(())
    }

    pub fn dispatch(&mut self, event: DataEvent) -> AppResult<()> {
        self.is_loading = true;
        self.ui.is_loading = true;

        self.data_handler.dispatch(event)?;
        Ok(())
    }

    pub fn render(ui: &mut Ui, frame: &mut Frame) {
        let rect = frame.area();
        frame.render_widget(ui, rect);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            _ => {
                if let Some(callback) = self.ui.handle_key_event(key) {
                    match (callback)(self) {
                        Ok(_) => {}
                        Err(_) => {
                            self.is_running = false;
                        }
                    }
                }
            }
        }
    }

    pub fn handle_mouse_event(&mut self, _event: MouseEvent) {}

    pub fn handle_tick_event(&mut self, _tick: Tick) {
        assert_eq!(self.ui.is_loading, self.is_loading);
        if self.is_loading {
            if let Ok(event) = self.data_handler.next() {
                match event {
                    DataEvent::Complete => {
                        self.is_loading = false;
                        self.ui.is_loading = false;
                    }
                    DataEvent::Updating(message) => {
                        self.ui.loading_msg = message;
                    }
                    DataEvent::Deleting(message) => {
                        self.ui.loading_msg = message;
                    }
                    DataEvent::ReloadedFeeds(feeds) => {
                        let feed_ids: Vec<i64> = feeds.iter().map(|f| f.id).collect();
                        self.ui.update_feeds(feeds);
                        self.dispatch(DataEvent::ReloadEntries(feed_ids))
                            .expect("Couldn't send ReloadEntries event");
                    }
                    DataEvent::ReloadedEntries(entries) => {
                        self.ui.next_entries();
                        self.ui.update_entries(entries);
                    }
                    DataEvent::Error(_) => {
                        self.is_running = false;
                    }
                    _ => {}
                }
            }
        }
        self.ui.update();
    }
}
