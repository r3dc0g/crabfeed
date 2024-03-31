use ratatui::layout::Rect;

let DEFAULT_ROUTE = Route {
    id: RouteId::Feeds,

}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Feeds,
    Entries,
    Entry
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Feeds,
    Entries,
    Entry,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
}

pub struct App {
    navigation_stack: Vec<Route>,
    // pub user_config: UserConfig,
    pub selected_feed_index: Option<usize>,
    pub selected_entry_index: Option<usize>,
    pub size: Rect,
    pub is_loading: bool,
    // io_tx: Option<Sender<IoEvent>>,
    pub is_fetching_current_feed: bool,
    pub dialog: Option<String>,
    pub confirm: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_feed_index: None,
            selected_entry_index: None,
            is_loading: false,
            is_fetching_current_feed: false,
            dialog: None,
            confirm: false,
        }
    }


}

impl App {
    pub fn new() -> Self {
        App::default()
    }
}
