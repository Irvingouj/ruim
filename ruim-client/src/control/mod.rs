use std::collections::HashMap;

use ratatui::widgets::StatefulWidget;

use crate::ui::{FocusableStatefulWidget, IdentifiableStatefulWidget};
pub mod state_type;
pub use state_type::*;

/// Each focusable stateful component (FSC) is identified by a unique ID.
/// THe FSC should store its state in the stateful_component_states field of the State struct.

#[derive(Debug, Clone)]
pub struct State {
    pub page: Page,
    pub stateful_component_states: HashMap<String, ComponentStateType>,
    currently_focused_component: Option<String>,
    current_page: Page,
    current_page_focusable_stateful_components: Vec<String>,
    mode: AppMode,
}

impl State {
    pub(crate) fn new_page(&mut self, page: Page) {
        self.current_page = page;
        self.current_page_focusable_stateful_components.clear();
    }

    pub(crate) fn quit(&mut self) {
        self.mode = AppMode::Quit;
    }

    /// Will only work if a component is focused
    pub(crate) fn edit(&mut self) {
        if self.currently_focused_component.is_some() {
            self.mode = AppMode::Running(RuningMode::Editing);
        }
    }

    pub(crate) fn normal(&mut self) {
        self.mode = AppMode::Running(RuningMode::Normal);
    }

    pub(crate) fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub(crate) fn get_state_mut(&mut self, id: &str) -> Option<&mut ComponentStateType> {
        self.stateful_component_states.get_mut(id)
    }

    pub(crate) fn currently_focused_component_id(&self) -> Option<String> {
        self.currently_focused_component.as_ref().cloned()
    }

    pub(crate) fn currently_focused_component_mut_state(
        &mut self,
    ) -> Option<&mut ComponentStateType> {
        self.currently_focused_component
            .as_ref()
            .and_then(|id| self.stateful_component_states.get_mut(id))
    }

    pub(crate) fn register_identifiable_widget<'a, T: IdentifiableStatefulWidget>(
        &'a mut self,
        id: &str,
    ) -> RegisterComponent<'a, T> {
        RegisterComponent {
            id: id.to_string(),
            state: self,
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn next_focusable_component(&mut self) {
        match &self.currently_focused_component {
            Some(id) => {
                let index = self
                    .current_page_focusable_stateful_components
                    .iter()
                    .position(|i| i == id)
                    .expect("Focused component not found");

                let next_index =
                    (index + 1) % self.current_page_focusable_stateful_components.len();
                let next_id = self.current_page_focusable_stateful_components[next_index].clone();
                self.currently_focused_component = Some(next_id);
            }
            None => {
                if !self.current_page_focusable_stateful_components.is_empty() {
                    self.currently_focused_component =
                        Some(self.current_page_focusable_stateful_components[0].clone());
                } else {
                    self.currently_focused_component = None;
                }
            }
        }
    }

    pub(crate) fn is_running(&self) -> bool {
        matches!(self.mode, AppMode::Running(_))
    }
}

pub struct RegisterComponent<'a, Widget>
where
    Widget: IdentifiableStatefulWidget,
{
    id: String,
    state: &'a mut State,
    _phantom: std::marker::PhantomData<&'a Widget>,
}

impl<'a, Widget> RegisterComponent<'a, Widget>
where
    Widget: IdentifiableStatefulWidget,
{
    pub(crate) fn configure_component<F>(self, configure_widget: F) -> GetCopyOfState<'a, Widget>
    where
        F: FnOnce(Widget) -> Widget,
    {
        let widget = Widget::new_with_id(&self.id);
        let widget = configure_widget(widget);
        GetCopyOfState {
            id: self.id,
            state: self.state,
            widget,
        }
    }
}

pub struct GetCopyOfState<'a, T>
where
    T: IdentifiableStatefulWidget,
{
    id: String,
    state: &'a mut State,
    widget: T,
}

impl<'a, Widget, StateType> GetCopyOfState<'a, Widget>
where
    Widget: IdentifiableStatefulWidget,
    Widget: StatefulWidget<State = StateType>,
    StateType: Default + Into<ComponentStateType>,
    ComponentStateType: AsMut<StateType>,
{
    pub fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let state = self
            .state
            .stateful_component_states
            .entry(self.id.clone())
            .or_insert(StateType::default().into());

        self.widget.render(area, buf, state.as_mut());
    }
}

impl<'a, Widget> GetCopyOfState<'a, Widget>
where
    Widget: IdentifiableStatefulWidget + FocusableStatefulWidget,
{
    pub fn focus(mut self) -> Self {
        self.state
            .current_page_focusable_stateful_components
            .push(self.id.clone());

        if let Some(id) = &self.state.currently_focused_component {
            if id == &self.id {
                self.widget = self.widget.focus(true);
            }
        }
        self
    }
}
