use std::time::Duration;

use crossterm::event::Event;
use ratatui::{backend::Backend, widgets::Widget, Frame, Terminal};

use crate::{
    control::{self, AppMode, State},
    term::next_event,
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
        tracing::info!("new ruim app created");
        Self {
            ui: Ui,
            state: State::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        let (sender, receiver) = flume::unbounded();
        let sender_clone = sender.clone();

        ctrlc::set_handler(move || {
            sender_clone.send(RuimEvent::CtrlC).unwrap();
        })?;

        std::thread::spawn(move || loop {
            if let Ok(Some(event)) = next_event(Duration::from_millis(50)) {
                match event {
                    Event::Key(key) => {
                        sender.send(RuimEvent::Key(key)).unwrap();
                    }
                    _ => todo!(),
                }
            }
        });

        while self.state.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(&receiver)?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self, receiver: &flume::Receiver<RuimEvent>) -> anyhow::Result<()> {
        if let Ok(event) = receiver.recv() {
            tracing::info!(?event, "event received");
            match event {
                RuimEvent::CtrlC => self.state.quit(),
                RuimEvent::Key(key) => self.handle_key_press(key),
            }
        }
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let AppMode::Running(running_mode) = self.state.mode() else {
            tracing::error!("App mode is not running");
            return;
        };
        let (area, buf) = self.ui.render_outer_frame(area, buf, running_mode);

        match &self.state.page {
            control::Page::Login => {
                self.state.new_page(control::Page::Login);
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

#[derive(Debug)]
pub(crate) enum RuimEvent {
    CtrlC,
    Key(crossterm::event::KeyEvent),
}
