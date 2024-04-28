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
            handle_esc(app);
        }

        _ => handle_block_event(key, app),
    }
}

fn handle_block_event(key: Key, app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::Feeds => {
            feeds::handle(key, app);
        }
        ActiveBlock::Entries => {
            entries::handle(key, app);
        }
        ActiveBlock::Entry => {
            entry::handle(key, app);
        }
    }
}

fn handle_esc(app: &mut App) {
    let last_route = app.get_current_route().clone();
    app.pop_navigation_stack();
    if last_route  == *app.get_current_route() {
        close_app().unwrap();
    }
}

