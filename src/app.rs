use crate::config::Settings;
use crate::data::data::{Cache, DataEvent, DataHandler};
use crate::error::Error;
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

#[derive(Debug)]
pub enum AppEvent {
    Complete,
    Error(Box<Error>),
    DisplayMsg(String),
    FeshData(Cache),
}

pub struct App {
    pub is_running: bool,
    pub is_loading: bool,
    pub ui: Ui,
    pub data_handler: DataHandler,
    pub running_data_calls: u16,
}

impl App {
    pub fn new(config: Settings) -> Self {
        App {
            is_running: true,
            is_loading: false,
            ui: Ui::new(config.clone()),
            data_handler: DataHandler::new(config.database_url.clone()),
            running_data_calls: 0,
        }
    }

    pub fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let event_handler = EventHandler::new();
        let mut tui = Tui::new(backend, event_handler)?;

        self.dispatch(DataEvent::Refresh)?;

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
        self.dispatch(DataEvent::Abort)?;
        Ok(())
    }

    pub fn dispatch(&mut self, event: DataEvent) -> AppResult<()> {
        self.running_data_calls += 1;
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
                    AppEvent::Complete => {
                        self.running_data_calls -= 1;
                        if self.running_data_calls == 0 {
                            self.is_loading = false;
                            self.ui.is_loading = false;
                        }
                    }
                    AppEvent::DisplayMsg(message) => {
                        self.ui.loading_msg = message;
                    }
                    AppEvent::FeshData(data) => {
                        self.ui.update_feeds(data.feeds);
                        self.ui.update_entries(data.entries);
                    }
                    AppEvent::Error(_) => {
                        self.is_running = false;
                    }
                }
            }
        }
        self.ui.update();
    }
}
