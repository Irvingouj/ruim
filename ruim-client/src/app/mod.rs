use std::time::Duration;

use crossterm::event::{Event, KeyEventKind};
use ratatui::{backend::Backend, widgets::Widget, Frame, Terminal};

use crate::{
    control::{self, State},
    term,
    ui::Ui,
};

mod key_events;

#[derive(Debug)]
pub struct App {
    ui: Ui,
    state: State,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: Ui,
            state: State::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        matches!(self.state.mode, control::AppMode::Running(..))
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }
    

}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let (area,buf) = self.ui.render_outer_frame(area,buf);
        match &self.state.page {
            control::Page::Login { .. } => {
                self.ui.render_login(&mut self.state, area, buf);
            }
            control::Page::Home => {
                // render home page
            }
            control::Page::Settings => {
                // render settings page
            }
            control::Page::Chat => {
                // render chat page
            }
        }
    }
}
