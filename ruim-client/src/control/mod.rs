use std::{collections::HashMap, hash::Hash};

use ratatui::widgets::{StatefulWidget, Widget};

use crate::ui::{FocusableStatefulWidget, IdentifiableStatefulWidget};

/// Each focusable stateful component (FSC) is identified by a unique ID.
/// THe FSC should store its state in the stateful_component_states field of the State struct.

#[derive(Debug, Clone)]
pub struct State {
    pub page: Page,
    pub mode: AppMode,
    pub stateful_component_states: HashMap<String, ComponentStateType>,
    currently_focused_component: Option<String>,
    current_page: Page,
    current_page_focusable_stateful_components: Vec<String>,
}

impl State {
    pub(crate) fn new_page(&mut self, page: Page) {
        self.current_page = page;
        self.current_page_focusable_stateful_components.clear();
    }

    pub(crate) fn register_identifiable_widget<'a, 'b, T: IdentifiableStatefulWidget>(
        &'a mut self,
        id: &'b str,
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

#[derive(Debug, Clone)]
pub enum ComponentStateType {
    TextInput(String),
    CheckBox(bool),
}

impl From<String> for ComponentStateType {
    fn from(s: String) -> Self {
        ComponentStateType::TextInput(s)
    }
}

impl From<bool> for ComponentStateType {
    fn from(b: bool) -> Self {
        ComponentStateType::CheckBox(b)
    }
}

impl AsMut<String> for ComponentStateType {
    fn as_mut(&mut self) -> &mut String {
        match self {
            ComponentStateType::TextInput(s) => s,
            _ => panic!("Invalid type"),
        }
    }
}

impl AsMut<bool> for ComponentStateType {
    fn as_mut(&mut self) -> &mut bool {
        match self {
            ComponentStateType::CheckBox(b) => b,
            _ => panic!("Invalid type"),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            page: Page::Login,
            mode: AppMode::Running(RuningMode::Normal),
            stateful_component_states: HashMap::new(),
            currently_focused_component: None,
            current_page: Page::Login,
            current_page_focusable_stateful_components: vec![],
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Page {
    Login,
    Home,
    Settings,
    Chat,
}

#[derive(Debug, Clone)]
pub enum AppMode {
    Running(RuningMode),
    Destroy,
    Quit,
}

#[derive(Debug, Clone)]
pub enum RuningMode {
    Normal,
    Editing,
}
