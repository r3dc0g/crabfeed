mod feeds;
mod entries;
mod entry;
mod common_key_events;

use crate::app::ActiveBlock;
use crate::{app::App, event::Key};
use crate::close_app;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            handle_esc();
        }

        _ => handle_block_event(key, app),
    }
}

fn handle_block_event(key: Key, app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::Feeds => {
            feeds::handle(key, app);
        }
        // TODO: Add other blocks
        _ => {}
    }
}

fn handle_esc() {
    close_app().unwrap();
}

