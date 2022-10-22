use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    props::{Alignment, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    Component, MockComponent, NoUserEvent,
};

use super::AppMsg;

#[derive(MockComponent)]
pub struct StatusBar {
    component: Container,
}

impl StatusBar {
    pub fn new() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(
                Label::default()
                    .alignment(Alignment::Left)
                    .text("Press ESC to quit"),
            ),
            Box::new(Label::default().alignment(Alignment::Right).text("Welcome")),
        ];

        StatusBar {
            component: Container::default()
                .children(children)
                .background(Color::LightGreen)
                .foreground(Color::LightGreen)
                .borders(Borders::default())
                .layout(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Percentage(50), // Left label
                                Constraint::Percentage(50), // Right label
                            ]
                            .as_ref(),
                        ),
                ),
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, NoUserEvent> for StatusBar {
    fn on(&mut self, _ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        Some(AppMsg::None)
    }
}
