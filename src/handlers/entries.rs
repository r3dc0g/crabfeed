use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            let entries = &mut app.entry_items;

            let next_index = common_key_events::on_down_press_handler(&entries, app.selected_entry_index);
            app.selected_entry_index = Some(next_index);
            app.set_entry(app.entry_items[app.selected_entry_index.unwrap_or(0)].1);
        }
        k if common_key_events::up_event(k) => {
            let entries = &mut app.entry_items;

            let next_index = common_key_events::on_up_press_handler(&entries, app.selected_entry_index);
            app.selected_entry_index = Some(next_index);
            app.set_entry(app.entry_items[app.selected_entry_index.unwrap_or(0)].1);
        }
        k if common_key_events::left_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Feeds);
        }
        k if common_key_events::select_event(k) => {
            app.set_current_route(RouteId::Entry, ActiveBlock::Entry);
            app.update_link_items(app.entry_items[app.selected_entry_index.unwrap_or(0)].1);
        }
        _ => {}
    }
}
