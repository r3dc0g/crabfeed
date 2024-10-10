pub mod app;
pub mod db;
pub mod error;
pub mod event;
pub mod network;
pub mod prelude;
pub mod schema;
pub mod time;
pub mod tui;
pub mod ui;
pub mod config;

use error::Error;
pub type AppResult<T> = core::result::Result<T, Error>;
