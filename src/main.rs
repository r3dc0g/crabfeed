mod db;
mod schema;
mod prelude;
mod error;
mod ui;
mod tui;
mod app;
mod network;
mod event;
// mod handlers;
mod time;

use error::Error;

use crate::app::App;

pub(crate) type AppResult<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> AppResult<()> {

    App::new().run()?;

    Ok(())
}

