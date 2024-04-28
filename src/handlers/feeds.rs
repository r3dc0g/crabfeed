use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            let feeds = &mut app.feed_items;

            let next_index = common_key_events::on_down_press_handler(&feeds, app.selected_feed_index);
            app.selected_feed_index = Some(next_index);
            app.update_entry_items(app.feed_items[app.selected_feed_index.unwrap_or(0)].1);
        }
        k if common_key_events::up_event(k) => {
            let feeds = &mut app.feed_items;

            let next_index = common_key_events::on_up_press_handler(&feeds, app.selected_feed_index);
            app.selected_feed_index = Some(next_index);
            app.update_entry_items(app.feed_items[app.selected_feed_index.unwrap_or(0)].1);
        }
        k if common_key_events::right_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Entries);
        }
        _ => {}
    }
}
