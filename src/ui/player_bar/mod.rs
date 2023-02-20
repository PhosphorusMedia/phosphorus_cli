use core::song::SongDetails;

use tui_realm_stdlib::{Container, Label, Phantom, ProgressBar};
use tuirealm::{
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent,
};

use super::{event::UserEvent, AppMsg};

const LEFT_LABEL: usize = 0;
const CURRENT_TIME: usize = 1;
const PROGRESS_INDICATOR: usize = 2;
const LIMIT_TIME: usize = 3;

#[derive(MockComponent)]
pub struct PlayerBar {
    component: Container,
}

impl PlayerBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn set_song(&mut self, details: &SongDetails) {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        children
            .get_mut(LEFT_LABEL)
            .unwrap()
            .attr(Attribute::Text, AttrValue::String(details.name().into()));
        children
            .get_mut(CURRENT_TIME)
            .unwrap()
            .attr(Attribute::Text, AttrValue::String("0".into()));
        children.get_mut(PROGRESS_INDICATOR).unwrap().attr(
            Attribute::Value,
            AttrValue::Payload(tuirealm::props::PropPayload::One(
                tuirealm::props::PropValue::F64(0.0),
            )),
        );
        children.get_mut(LIMIT_TIME).unwrap().attr(
            Attribute::Text,
            AttrValue::String(details.duration_str().unwrap_or("--:--".into())),
        );
    }
}

impl Default for PlayerBar {
    fn default() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Label::default().alignment(Alignment::Left).text("--")),
            Box::new(Label::default().alignment(Alignment::Right).text("--:--")),
            Box::new(
                ProgressBar::default()
                    .progress(0.0)
                    .borders(Borders::default().sides(BorderSides::empty()))
                    .foreground(Color::Red),
            ),
            Box::new(Label::default().alignment(Alignment::Left).text("--:--")),
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
                                Constraint::Percentage(20),
                                Constraint::Percentage(5),
                                Constraint::Percentage(50),
                                Constraint::Percentage(5),
                                Constraint::Percentage(20),
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
            Event::User(UserEvent::PlaySong(song)) => {
                self.set_song(song.details());
            }
            _ => {}
        }

        None
    }
}
