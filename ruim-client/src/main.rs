use app::App;

pub mod app;
pub mod term;
pub mod ui;
pub mod control;

fn main() -> anyhow::Result<()> {
    let terminal = &mut term::init()?;
    App::new().run(terminal)?;
    term::restore()?;
    Ok(())
}
