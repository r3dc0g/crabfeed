use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{app::App, config::Settings, AppResult};

mod add;
mod components;
mod entries;
mod entry;
mod feeds;
pub mod ui;
pub mod util;

pub type UiCallback = Box<dyn Fn(&mut App) -> AppResult<()>>;

pub trait View {
    fn render(&self, area: Rect, buf: &mut Buffer, config: &Settings);

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback>;
}
