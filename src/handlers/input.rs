use crate::app::Route;
use crate::network::IOEvent;
use crate::{app::App, event::Key};

pub fn handle(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            let last_route = app.pop_navigation_stack().unwrap_or(Route::default());
            app.set_current_route(last_route.id, last_route.active_block);
        }
        Key::Enter => {
            if let Some(io_tx) = app.io_tx.as_ref() {
                if !app.input.is_empty() {
                    let url = app.input.iter().collect::<String>();
                    io_tx.send(IOEvent::AddFeed(url)).unwrap();
                    app.clear_input();
                    let mut last_route = app.pop_navigation_stack().unwrap_or(Route::default());
                    last_route = app.pop_navigation_stack().unwrap_or(Route::default());
                    app.set_current_route(last_route.id, last_route.active_block);
                }
            }
        }
        Key::Left => {
            if app.input_i > 0 {
                app.input_i -= 1;
                app.input_cursor_position -= 1;
            }
        }
        Key::Right => {
            if app.input_i < app.input.len() {
                app.input_i += 1;
                app.input_cursor_position += 1;
            }
        }
        Key::Char(c) => {
            app.input.insert(app.input_i, c);
            app.input_i += 1;
            app.input_cursor_position += 1;
        }
        Key::Backspace => {
            if !app.input.is_empty() && app.input_i > 0 {
                app.input.remove(app.input_i - 1);
                app.input_i -= 1;
                app.input_cursor_position -= 1;
            }
        }
        Key::Delete => {
            if !app.input.is_empty() && app.input_i < app.input.len() {
                app.input.remove(app.input_i);
            }
        }
        _ => {}
    }
}

