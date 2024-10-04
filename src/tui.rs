use std::{io, panic};

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{layout::Rect, prelude::Backend, Terminal};

use crate::{app::App, event::EventHandler, ui::ui::Ui, AppResult};


pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub event_handler: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(backend: B, event_handler: EventHandler) -> AppResult<Self> {
        let terminal = Terminal::new(backend)?;
        let mut tui = Self {
            terminal,
            event_handler,
        };
        tui.init()?;
        Ok(tui)
    }

    fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, ui: &mut Ui) -> AppResult<()> {
        self.terminal
            .draw(|f| App::render(ui, f))?;
        Ok(())
    }

    pub fn resize(&mut self, rect: Rect) -> AppResult<()> {
        self.terminal.resize(rect)?;
        Ok(())
    }

    pub fn reset() -> AppResult<()> {
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.clear()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
