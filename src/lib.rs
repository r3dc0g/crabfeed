pub mod app;
pub mod config;
pub mod db;
pub mod error;
pub mod event;
pub mod network;
pub mod prelude;
pub mod time;
pub mod tui;
pub mod ui;

use error::Error;
pub type AppResult<T> = core::result::Result<T, Error>;
