use std::{
    io::{self, stdout},
    time::Duration,
};

use anyhow::Context;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn init() -> anyhow::Result<Terminal<impl Backend>> {
    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    enable_raw_mode().context("enable raw mode")?;
    stdout()
        .execute(EnterAlternateScreen)
        .context("enter alternate screen")?;
    Ok(terminal)
}

pub fn restore() -> anyhow::Result<()> {
    disable_raw_mode().context("disable raw mode")?;
    stdout()
        .execute(LeaveAlternateScreen)
        .context("leave alternate screen")?;
    Ok(())
}

pub fn next_event(timeout: Duration) -> anyhow::Result<Option<Event>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    let event = event::read()?;
    Ok(Some(event))
}
