use app::App;
use dotenv::dotenv;
use logging::initialize_logging;

pub mod app;
pub mod term;
pub mod ui;
pub mod control;
pub mod logging;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    initialize_logging().expect("Failed to initialize logging");
    let terminal = &mut term::init()?;
    App::new().run(terminal)?;
    term::restore()?;
    Ok(())
}
