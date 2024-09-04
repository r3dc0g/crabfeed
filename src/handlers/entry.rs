use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    // TODO: Add a match statement to handle the key events
    match key {
        k if common_key_events::left_event(k) => {
            app.entry_line_index = 0;
            app.update_entry_items(app.feed_items[app.feed_list_state.selected().unwrap_or(0)].1);
            app.set_current_route(RouteId::Home, ActiveBlock::Entries);
        }

        k if common_key_events::down_event(k) => {
            app.entry_line_index += 1;
        }

        k if common_key_events::up_event(k) => {
            if app.entry_line_index > 0 {
                app.entry_line_index -= 1;
            }
        }
        _ => {}
    }
}
