use self::components::input::{TextInput, LOGIN_INPUT_ID, PASSWORD_INPUT_ID};
use crate::control::Page;
use ratatui::layout::{Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
mod components;
pub use components::traits::*;
#[derive(Debug, Default)]
pub struct Ui;
impl Ui {
    pub(crate) fn render_login(
        &self,
        state: &mut crate::control::State,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        state.new_page(Page::Login);
        // Define the overall layout for the login box
        let login_box = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Login");

        // Render the login box
        login_box.render(area, buf);

        let inner = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };
        // Divide the inner area into two parts for username and password inputs
        let chunks = ratatui::prelude::Layout::default()
            .direction(ratatui::prelude::Direction::Vertical)
            .constraints(
                [
                    ratatui::prelude::Constraint::Percentage(50),
                    ratatui::prelude::Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(inner);

        state
            .register_identifiable_widget(LOGIN_INPUT_ID)
            .configure_component(|user_input: TextInput| user_input.title("Username"))
            .focus()
            .render(chunks[0], buf);

        state
            .register_identifiable_widget(PASSWORD_INPUT_ID)
            .configure_component(|password_input: TextInput| password_input.title("Password"))
            .focus()
            .render(chunks[1], buf);
    }

    pub(crate) fn render_outer_frame<'a>(
        &'a self,
        area: Rect,
        buf: &'a mut ratatui::prelude::Buffer,
    ) -> (Rect, &mut ratatui::prelude::Buffer) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(
                [
                    ratatui::prelude::Constraint::Percentage(90),
                    ratatui::prelude::Constraint::Fill(1),
                ]
                .as_ref(),
            )
            .split(area);

        let inner = layout[0];

        Paragraph::new("Ruim")
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
            )
            .render(inner, buf);

        return (inner, buf);
    }
}
