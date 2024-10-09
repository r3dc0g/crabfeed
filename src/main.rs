use crabfeed::app::App;
use crabfeed::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    App::new().run()?;

    Ok(())
}
