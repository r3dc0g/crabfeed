mod feeds;
mod entries;
mod entry;
mod input;
mod common_key_events;

use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::close_app;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            handle_esc(app);
        }

        Key::Ctrl('a') => {
            let current_route = app.get_current_route().clone();
            app.push_navigation_stack(current_route.id, current_route.active_block);
            app.set_current_route(RouteId::Home, ActiveBlock::Input);
        }

        Key::Ctrl('r') => {
            app.update_feed_items();
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
        ActiveBlock::Input => {
            input::handle(key, app);
        }
    }
}

fn handle_esc(app: &mut App) {
    app.clear_input();
    let last_route = app.get_current_route().clone();
    app.pop_navigation_stack();
    if last_route  == *app.get_current_route() {
        close_app().unwrap();
    }
}

