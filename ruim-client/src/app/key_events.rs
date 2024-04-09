use crossterm::event::KeyCode;

use crate::control::{AppMode, RuningMode};

use super::{
    key_actions::{KeyAction, KeyIdentifier},
    App,
};

impl App {
    pub(super) fn handle_key_press(&mut self, key_event: crossterm::event::KeyEvent) {
        let AppMode::Running(running_mode) = self.state.mode() else {
            return;
        };

        if matches!(running_mode, RuningMode::Editing)
            && matches!(key_event.code, KeyCode::Char(..))
        {
            let KeyCode::Char(c) = key_event.code else {
                unreachable!()
            };

            if let Some(state) = self.state.currently_focused_component_mut_state() {
                state.push_char(c);
            }
        }

        let key_identifier = KeyIdentifier::builder()
            .key_code(key_event.code)
            .modifiers(key_event.modifiers)
            .build();

        let key_action = self.functional_keys.get(&key_identifier);
        let Some(key_action) = key_action else {
            tracing::debug!("no key action found for {:?}", key_identifier);
            return;
        };

        key_action.action(&mut self.state);
    }
}

pub(crate) fn functional_key_actions(
) -> std::collections::HashMap<KeyIdentifier, super::key_actions::KeyAction> {
    let mut key_actions = std::collections::HashMap::new();

    key_actions.insert(
        KeyIdentifier::builder()
            .key_code(KeyCode::Char('q'))
            .build(),
        KeyAction::builder()
            .key_short_help_text("quit")
            .key_long_help_text("Quit the application")
            .activate_in_mode(vec![RuningMode::Normal])
            .action(|state| {
                state.quit();
            })
            .build(),
    );

    key_actions.insert(
        KeyIdentifier::builder().key_code(KeyCode::Tab).build(),
        KeyAction::builder()
            .key_short_help_text("next focus")
            .key_long_help_text("Move focus to the next component")
            .activate_in_mode(vec![RuningMode::Normal])
            .action(|state| {
                state.next_focusable_component();
            })
            .build(),
    );

    key_actions.insert(
        KeyIdentifier::builder()
            .key_code(KeyCode::Char('e'))
            .build(),
        KeyAction::builder()
            .key_short_help_text("edit")
            .key_long_help_text("Edit the currently focused component")
            .activate_in_mode(vec![RuningMode::Normal])
            .action(|state| {
                state.edit();
            })
            .build(),
    );

    key_actions.insert(
        KeyIdentifier::builder().key_code(KeyCode::Esc).build(),
        KeyAction::builder()
            .key_short_help_text("back to normal")
            .key_long_help_text("Return to normal mode")
            .activate_in_mode(vec![RuningMode::Editing])
            .action(|state| {
                state.normal();
            })
            .build(),
    );
    key_actions
}
