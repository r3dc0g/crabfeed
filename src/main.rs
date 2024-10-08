mod app;
mod db;
mod error;
mod event;
mod network;
mod prelude;
mod schema;
mod time;
mod tui;
mod ui;

use error::Error;

use crate::app::App;

pub(crate) type AppResult<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> AppResult<()> {
    App::new().run()?;

    Ok(())
}
