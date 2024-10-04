use crate::event::{EventHandler, TerminalEvent};
use crate::network::NetworkHandler;
use crate::time::{SystemTimeTick, Tick, TICK_RATE};
use crate::tui::Tui;
use crate::ui::ui::Ui;
use crate::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
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
    Input,
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
    pub network_handler: NetworkHandler,
}

impl App {

    pub fn new() -> Self {
        App {
            is_running: true,
            is_loading: true,
            ui: Ui::new(),
            network_handler: NetworkHandler::new(),
        }
    }

    pub fn run(&mut self) -> AppResult<()> {

        let backend = CrosstermBackend::new(io::stdout());
        let event_handler = EventHandler::new();
        let mut tui = Tui::new(backend, event_handler)?;

        while self.is_running {
            match tui.event_handler.next()? {
                TerminalEvent::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        self.is_running = false;
                    }
                    self.handle_key_event(key);
                }
                TerminalEvent::Mouse(mouse) => {
                    self.handle_mouse_event(mouse);
                }
                TerminalEvent::Resize(w, h) => {
                    self.handle_resize_event(w, h);
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
                        Ok(_) => {},
                        Err(_) => {
                            self.is_running = false;
                        }
                    }
                }
            }
        }
    }

    pub fn handle_mouse_event(&mut self, event: MouseEvent) {

    }

    pub fn handle_resize_event(&mut self, w: u16, h: u16) {

    }

    pub fn handle_tick_event(&mut self, _tick: Tick) {
        self.ui.update();
    }

}
