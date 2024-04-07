use crossterm::event::KeyCode;

use crate::control::{AppMode, RuningMode};

use super::App;

impl App {
    pub(super) fn handle_key_press(&mut self, key: crossterm::event::KeyEvent) {
        let AppMode::Running(running_mode) = self.state.mode() else {
            return;
        };

        match running_mode {
            RuningMode::Normal => {
                self.handle_normal_mode_key_press(key);
            }
            RuningMode::Editing => {
                self.handle_editing_mode_key_press(key);
            }
        }
    }

    fn handle_normal_mode_key_press(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('i') => {
                self.state.edit();
            }
            KeyCode::Char('q') => {
                self.state.quit();
            }
            KeyCode::Char('c') => {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                {
                    self.state.quit();
                }
            }
            KeyCode::Tab => {
                self.state.next_focusable_component();
            }
            _ => {}
        }
    }

    fn handle_editing_mode_key_press(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.state.normal();
            }
            KeyCode::Char(char) => {
                let focused_component = self.state.currently_focused_component_id();
                if let Some(focused_component) = focused_component {
                    self.state
                        .get_state_mut(focused_component.as_ref())
                        .map(|state_ref_mut| state_ref_mut.push_char(char))
                        .unwrap_or_else(|| {
                            tracing::warn!("No focused component found");
                            self.state.normal();
                        })
                } else {
                    tracing::warn!("No focused component found");
                    self.state.normal();
                }
            }
            _ => {}
        }
    }
}
