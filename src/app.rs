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
    pub network_handler: DataHandler,
}

impl App {
    pub fn new() -> Self {
        App {
            is_running: true,
            is_loading: false,
            ui: Ui::new(),
            network_handler: DataHandler::new(),
        }
    }

    pub fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let event_handler = EventHandler::new();
        let mut tui = Tui::new(backend, event_handler)?;

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
        tui.exit()?;
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
            if let Ok(event) = self.network_handler.next() {
                match event {
                    DataEvent::Complete => {
                        self.ui.update_feeds();
                        self.is_loading = false;
                        self.ui.is_loading = false;
                    }
                    DataEvent::Updating(message) => {
                        self.ui.loading_msg = message;
                    }
                    DataEvent::Deleting(message) => {
                        self.ui.loading_msg = message;
                    }
                    DataEvent::ReloadedFeeds(_feeds) => {
                        // Send new feeds to ui
                    }
                    DataEvent::ReloadedEntries(_entries) => {
                        // Send new entries to ui
                    }
                    _ => {}
                }
            }
        }
        self.ui.update();
    }
}
