use tui_realm_stdlib::Paragraph;
use tuirealm::{
    props::{Alignment, Color, TextSpan},
    Component, MockComponent, NoUserEvent,
};

use super::AppMsg;

#[derive(MockComponent)]
pub struct WelcomWindow {
    component: Paragraph,
}

impl WelcomWindow {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Default for WelcomWindow {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                //.title("Welcome in Phosphorus!", Alignment::Center)
                .text(&[
                    TextSpan::new("Welcome in Phosphorus!")
                        .bold()
                        .underlined()
                        .fg(Color::LightRed),
                    TextSpan::new(""),
                    TextSpan::new("Music from everywhere, music for everyone")
                        .italic()
                        .fg(Color::LightYellow),
                ])
                .alignment(Alignment::Center),
        }
    }
}

impl Component<AppMsg, NoUserEvent> for WelcomWindow {
    fn on(&mut self, _ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        Some(AppMsg::None)
    }
}
