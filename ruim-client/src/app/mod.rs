use std::{collections::HashMap, time::Duration};

use crossterm::event::Event;
use ratatui::{backend::Backend, widgets::Widget, Frame, Terminal};

use crate::{
    control::{self, AppMode, State},
    term::next_event,
    ui::Ui,
};

mod key_actions;
mod key_events;
#[derive(Debug)]
pub struct App {
    ui: Ui,
    state: State,
    functional_keys: HashMap<key_actions::KeyIdentifier, key_actions::KeyAction>,
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
            functional_keys: key_events::functional_key_actions(),
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

    pub(crate) fn help_text(&self) -> String {
        let mode = self.state.mode().to_string();
        let AppMode::Running(running_mode) = self.state.mode() else {
            tracing::error!("App mode is not running");
            return mode;
        };
        let keys = self
            .functional_keys
            .iter()
            .filter(|(_, action)| action.activate_in_mode().contains(running_mode))
            .map(|(key, action)| format!("{}[{}]", key, action.short_help_text(),))
            .collect::<Vec<String>>()
            .join(",");

        format!("{}:{}", mode, keys)
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let AppMode::Running(_) = self.state.mode() else {
            tracing::error!("App mode is not running");
            return;
        };
        let (area, buf) = self.ui.render_outer_frame(area, buf, self);

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
