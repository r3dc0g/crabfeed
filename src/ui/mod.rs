use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

use crate::{app::App, AppResult};

mod add;
mod components;
mod entries;
mod entry;
mod feeds;
pub mod ui;
mod util;

pub type UiCallback = Box<dyn Fn(&mut App) -> AppResult<()>>;
const SELECTED_STYLE: Style = Style::new().fg(Color::Red);
const UNSELECTED_STYLE: Style = Style::new();

pub trait View {
    fn render(&self, area: Rect, buf: &mut Buffer);

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<UiCallback>;
}
