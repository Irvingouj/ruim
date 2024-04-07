use ratatui::{layout::Layout, style, widgets::{Block, Borders, Paragraph, StatefulWidget, Widget}};

use crate::ui::{FocusableStatefulWidget, IdentifiableStatefulWidget};

pub const LOGIN_INPUT_ID: &str = "login_page_input";
pub const PASSWORD_INPUT_ID: &str = "password_page_input";

#[derive(Debug)]
pub struct TextInput {
    pub title: Option<String>,
    pub focused: bool,
    pub id: String,
}
impl TextInput {
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

}

impl StatefulWidget for TextInput {
    type State = String;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let paragraph = Paragraph::new(state.as_str());

        let mut block = Block::default().borders(Borders::ALL);

        if let Some(title) = &self.title {
             block = block.title(title.as_str());
        }

        if self.focused {
            block = block.border_style(style::Style::default().fg(style::Color::Yellow));
        }

        let paragraph = paragraph.block(block);
        let input_area = Layout::default()
            .constraints(vec![ratatui::prelude::Constraint::Length(3),ratatui::prelude::Constraint::Fill(1)])
            .split(area)[0];

        paragraph.render(input_area, buf);
    }
}


impl FocusableStatefulWidget for TextInput {
    fn focus(mut self, value:bool) -> Self {
        self.focused = value;
        self
    }
}

impl IdentifiableStatefulWidget for TextInput {
    fn id(&self) -> String {
        self.title.clone().unwrap_or_default()
    }

    fn new_with_id(id: &str) -> Self {
        Self {
            id: id.to_string(),
            focused: false,
            title: None,
        }
    }
}