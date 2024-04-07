use ratatui::widgets::StatefulWidget;

pub trait FocusableStatefulWidget: StatefulWidget {
    #[must_use]
    fn focus(self, value: bool) -> Self;
}

pub trait IdentifiableStatefulWidget: StatefulWidget {
    fn id(&self) -> String;
    fn new_with_id(id: &str) -> Self;
}
