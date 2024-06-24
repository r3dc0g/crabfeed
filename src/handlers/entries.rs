use crate::app::{ActiveBlock, RouteId};
use crate::db::mark_entry_read;
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            let entries = &mut app.entry_items;

            if !entries.is_empty() {
                let next_index = common_key_events::on_down_press_handler(&entries, app.selected_entry_index);
                app.selected_entry_index = Some(next_index);
                app.set_entry(app.entry_items[app.selected_entry_index.unwrap_or(0)].1.0);
            }

        }
        k if common_key_events::up_event(k) => {
            let entries = &mut app.entry_items;

            if !entries.is_empty() {
                let next_index = common_key_events::on_up_press_handler(&entries, app.selected_entry_index);
                app.selected_entry_index = Some(next_index);
                app.set_entry(app.entry_items[app.selected_entry_index.unwrap_or(0)].1.0);
            }

        }
        k if common_key_events::left_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Feeds);
        }
        k if common_key_events::select_event(k) || common_key_events::right_event(k) => {
            if app.selected_entry_index != None {
                mark_entry_read(app.entry_items[app.selected_entry_index.unwrap_or(0)].1.0).expect_err("Error marking entry as read");
                app.set_current_route(RouteId::Entry, ActiveBlock::Entry);
                app.update_link_items(app.entry_items[app.selected_entry_index.unwrap_or(0)].1.0);
            }
        }
        _ => {}
    }
}
