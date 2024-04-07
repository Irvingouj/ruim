use crossterm::event::KeyCode;

use crate::control::{AppMode, RuningMode};

use super::App;




impl App {
    pub(super) fn handle_key_press(&mut self, key: crossterm::event::KeyEvent) {
        let AppMode::Running(running_mode) =  self.state.mode() else {
            return;
        };

        match running_mode {
            RuningMode::Normal =>  {
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
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
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
            _ => {}
        }
    }
    
}