use crate::app::{ActiveBlock, RouteId};
use crate::{app::App, event::Key};
use crate::handlers::common_key_events;


pub fn handle(key: Key, app: &mut App) {
    // TODO: Add a match statement to handle the key events
    match key {
        k if common_key_events::left_event(k) => {
            app.set_current_route(RouteId::Home, ActiveBlock::Entries);
        }
        _ => {}
    }
}
