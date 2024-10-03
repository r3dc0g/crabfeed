mod tuihtml;

use crate::event::{EventHandler, TerminalEvent};
use crate::network::NetworkHandler;
use crate::time::{Tick, SystemTimeTick};
use crate::tui::Tui;
use crate::ui::ui::Ui;
use crate::AppResult;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use ratatui::prelude::CrosstermBackend;
use ratatui::Frame;
use std::io;

#[derive(Clone, Default, PartialEq, Debug)]
pub enum RouteId {
    #[default]
    Home,
    Entry,
}


#[derive(Debug, Default, Clone, PartialEq)]
pub enum ActiveBlock {
    #[default]
    Feeds,
    Entries,
    Entry,
    Input,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

impl Route {
    pub fn new(id: RouteId, active_block: ActiveBlock) -> Self {
        Self { id, active_block }
    }
}

pub struct App {
    pub is_running: bool,
    pub is_loading: bool,
    pub ui: Ui,
    pub network_handler: NetworkHandler,
}

impl App {

    pub fn new() -> Self {
        App {
            is_running: true,
            is_loading: true,
            ui: Ui::new(),
            network_handler: NetworkHandler::new(),
        }
    }

    pub fn run(&mut self) -> AppResult<()> {

        let backend = CrosstermBackend::new(io::stdout());
        let event_handler = EventHandler::new();
        let mut tui = Tui::new(backend, event_handler)?;

        while self.is_running {
            let now = Tick::now();

            match tui.event_handler.next()? {
                TerminalEvent::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        self.is_running = false;
                    }
                    self.handle_key_event(key);
                }
                TerminalEvent::Mouse(mouse) => {
                    self.handle_mouse_event(mouse);
                }
                TerminalEvent::Resize(w, h) => {
                    self.handle_resize_event(w, h);
                }
                TerminalEvent::Tick { tick } => {
                    self.handle_tick_event(tick);
                    tui.draw(&mut self.ui)?;
                }
            }
        }
        tui.exit()?;
        Ok(())
    }

    pub fn render(ui: &mut Ui, frame: &mut Frame) {
        let rect = frame.area();
        frame.render_widget(ui, rect);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            _ => {
                if let Some(callback) = self.ui.handle_key_event(key) {
                    match (callback)(self) {
                        Ok(_) => {},
                        Err(_) => {
                            self.is_running = false;
                        }
                    }
                }
            }
        }
    }

    pub fn handle_mouse_event(&mut self, event: MouseEvent) {

    }

    pub fn handle_resize_event(&mut self, w: u16, h: u16) {

    }

    pub fn handle_tick_event(&mut self, _tick: Tick) {
        self.ui.update();
    }

    // pub fn update_feed_items(&mut self) {
    //     let index = self.feed_list_state.selected().unwrap_or(0);
    //     self.loading_msg = "Reloading Feed Items...".to_string();
    //     self.is_loading = true;
    //     let feeds = get_feeds().unwrap_or(vec![]);
    //     self.feed_items = feeds.iter().map(|f| {
    //         (f.title.clone().unwrap_or("No title".to_string()).clone(), f.id)
    //     }).collect();
    //     if index < self.feed_items.len() {
    //         self.feed_list_state.select(Some(index));
    //     }
    //     else {
    //         self.feed_list_state.select(None);
    //     }
    //     self.feed_list_state.select(None);
    //     self.update_entry_items(0);
    //     self.is_loading = false;
    // }

    // pub fn update_entry_items(&mut self, feed_id: i32) {
    //     let entries = select_entries(feed_id).unwrap_or(vec![]);
    //     let index = self.entry_list_state.selected();
    //     self.entry_items = entries.iter().rev().map(|e| {
    //         (e.title.clone().unwrap_or("No Title".to_string()), (e.id, e.read.unwrap_or(false)))
    //     })
    //     .collect();
    //     if let Some(index) = index {
    //         if index < self.entry_items.len() {
    //             self.entry_list_state.select(Some(index));
    //         }
    //         else {
    //             self.entry_list_state.select(None);
    //         }
    //     }
    //     else {
    //         self.entry_list_state.select(None);
    //     }

    //     self.total_entries = self.entry_items.len();
    // }

    // pub fn set_entry(&mut self, entry_id: i32) {
    //     if let Ok(entry) = select_entry(&entry_id) {
    //         self.entry = Some(entry);
    //         return;
    //     }

    //     self.entry = None;
    // }

    // pub fn set_content(&mut self, content: Option<i32>) {

    //     match content {
    //         Some(content_id) => {
    //             if let Ok(content) = select_content(&content_id) {
    //                 let content_html = content.body.clone().unwrap_or("".to_string());

    //                 if let Ok(tui_content) = tuihtml::parse_html(content_html) {
    //                     self.entry_content = Some(tui_content);
    //                 }
    //                 else {
    //                     self.entry_content = None;
    //                 }
    //             }
    //         }

    //         None => {
    //             self.entry_content = None;
    //         }
    //     }

    // }

    // pub fn set_summary(&mut self) {

    //     if let Some(entry) = &self.entry {
    //         if let Some(summary_html) = &entry.summary {
    //             if let Ok(tui_summary) = tuihtml::parse_html(summary_html.to_string()) {
    //                 self.entry_summary = Some(tui_summary);
    //             }
    //             else {
    //                 self.entry_summary = None;
    //             }
    //         }
    //         else {
    //             self.entry_summary = None;
    //         }
    //     }
    //     else {
    //         self.entry_summary = None;
    //     }

    // }

    // pub fn set_entry_description(&mut self) {

    //     if let Some(entry) = &self.entry {
    //         if let Some(media_id) = entry.media_id {
    //             if let Ok(media) = select_media(media_id) {
    //                 if let Some(description) = media.description {
    //                     if let Ok(tui_description) = tuihtml::parse_html(description) {
    //                         self.entry_description = Some(tui_description);
    //                     }
    //                     else {
    //                         self.entry_description = None;
    //                     }
    //                 }
    //                 else {
    //                     self.entry_description = None;
    //                 }
    //             }
    //             else {
    //                 self.entry_description = None;
    //             }
    //         }
    //         else {
    //             self.entry_description = None;
    //         }
    //     }
    //     else {
    //         self.entry_description = None;
    //     }
    // }

    // pub fn update_link_items(&mut self, entry_id: i32) {
    //     let links = find_entry_links(entry_id).unwrap_or(vec![]);
    //     self.link_items = links.iter().map(|l| {
    //         (l.href.clone(), l.id)
    //     })
    //     .collect();
    // }

    // pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
    //     self.navigation_stack.push(Route { id: next_route_id, active_block: next_active_block});
    // }

    // pub fn pop_navigation_stack(&mut self) -> Option<Route> {
    //     if self.navigation_stack.len() > 1 {
    //         self.navigation_stack.pop()
    //     } else {
    //         None
    //     }
    // }

    // pub fn get_current_route(&self) -> &Route {
    //     self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    // }

    // pub fn set_current_route(&mut self, route: RouteId, active_block: ActiveBlock) {
    //     self.push_navigation_stack(route, active_block);
    // }

    // pub fn clear_input(&mut self) {
    //     self.input.clear();
    //     self.input_cursor_position = 0;
    //     self.input_i = 0;
    // }

}
