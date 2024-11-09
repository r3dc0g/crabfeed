use core::panic;

use crabfeed::app::App;
use crabfeed::config::get_configuration;
use crabfeed::AppResult;
use env_logger::Target;
use log::LevelFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = get_configuration()?;

    env_logger::builder()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();

    App::new(config).run()?;

    Ok(())
}
