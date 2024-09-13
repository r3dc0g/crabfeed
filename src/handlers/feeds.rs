use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;
use crate::network::IOEvent;


pub fn handle(key: Key, app: &mut App) {
    match key {

        k if common_key_events::down_event(k) => {
            let feeds = &mut app.feed_items;

            if !feeds.is_empty() {

                if let Some(index) = app.feed_list_state.selected() {
                    if index < app.feed_items.len() - 1 {
                        app.feed_list_state.select_next();
                    }
                    else {
                        app.feed_list_state.select_first();
                    }
                }
                else {
                    app.feed_list_state.select_first();
                }

                app.entry_list_state.select_first();
                app.update_entry_items(app.feed_items[app.feed_list_state.selected().unwrap_or(0)].1);
            }
        }

        k if common_key_events::up_event(k) => {
            let feeds = &mut app.feed_items;

            if !feeds.is_empty() {

                if let Some(index) = app.feed_list_state.selected() {
                    if index > 0 {
                        app.feed_list_state.select_previous();
                    }
                    else {
                        app.feed_list_state.select(Some(app.feed_items.len() - 1));
                    }
                }
                else {
                    app.feed_list_state.select(Some(app.feed_items.len() - 1));
                }

                app.entry_list_state.select_first();
                app.update_entry_items(app.feed_items[app.feed_list_state.selected().unwrap_or(0)].1);
            }

        }

        k if common_key_events::right_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Entries);
        }

        Key::Ctrl('d') => {
            app.loading_msg = format!("Deleting {}...", app.feed_items[app.feed_list_state.selected().unwrap_or(0)].0);
            app.is_loading = true;
            app.dispatch(IOEvent::DeleteFeed(app.feed_items[app.feed_list_state.selected().unwrap_or(0)].1));
        }

        _ => {}
    }
}
