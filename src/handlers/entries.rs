use crate::app::{ActiveBlock, RouteId};
use crate::db::mark_entry_read;
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            let entries = &mut app.entry_items;

            if !entries.is_empty() {
                if let Some(index) = app.entry_list_state.selected() {
                    if index < app.entry_items.len() - 1 {
                        app.entry_list_state.select_next();
                    }
                    else {
                        app.entry_list_state.select_first();
                    }
                }
                else {
                    app.entry_list_state.select_first();
                }

                app.set_entry(app.entry_items[app.entry_list_state.selected().unwrap_or(0)].1.0);
                app.set_content(match &app.entry { Some(entry) => entry.content_id.clone(), None => None });
            }

        }
        k if common_key_events::up_event(k) => {
            let entries = &mut app.entry_items;

            if !entries.is_empty() {

                if let Some(index) = app.entry_list_state.selected() {
                    if index > 0 {
                        app.entry_list_state.select_previous();
                    }
                    else {
                        app.entry_list_state.select(Some(app.entry_items.len() - 1));
                    }
                }
                else {
                    app.entry_list_state.select(Some(app.entry_items.len() - 1));
                }

                app.set_entry(app.entry_items[app.entry_list_state.selected().unwrap_or(0)].1.0);
                app.set_content(match &app.entry { Some(entry) => entry.content_id.clone(), None => None });
            }

        }
        k if common_key_events::left_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Feeds);
        }
        k if common_key_events::select_event(k) || common_key_events::right_event(k) => {
            mark_entry_read(app.entry_items[app.entry_list_state.selected().unwrap_or(0)].1.0).unwrap();
            app.set_current_route(RouteId::Entry, ActiveBlock::Entry);
            app.update_link_items(app.entry_items[app.entry_list_state.selected().unwrap_or(0)].1.0);
        }
        _ => {}
    }
}
