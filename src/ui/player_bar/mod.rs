use core::song::SongDetails;

use tui_realm_stdlib::{Container, Label, Phantom};
use tuirealm::{
    props::{BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    Component, MockComponent, Event,
};

use super::{event::UserEvent, AppMsg};

#[derive(MockComponent)]
pub struct PlayerBar {
    component: Container,
}

impl PlayerBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn set_song(&mut self, details: SongDetails) {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        children.get_mut(0).unwrap().attr(
            tuirealm::Attribute::Text,
            tuirealm::AttrValue::String(details.name().into()),
        );
        children.get_mut(1).unwrap().attr(
            tuirealm::Attribute::Text,
            tuirealm::AttrValue::String(details.name().into()),
        );
    }
}

impl Default for PlayerBar {
    fn default() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Label::default()),
            Box::new(Label::default()),
            Box::new(Phantom::default()),
        ];

        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::empty()))
                .foreground(Color::Reset)
                .children(children)
                .layout(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Percentage(25),
                                Constraint::Percentage(50),
                                Constraint::Percentage(25),
                            ]
                            .as_ref(),
                        ),
                ),
        }
    }
}

impl Component<AppMsg, UserEvent> for PlayerBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        match ev {
            Event::User(UserEvent::PlaySong(details)) => {
                self.set_song(details);
            }
            _ => {},
        }

        None
    }
}
