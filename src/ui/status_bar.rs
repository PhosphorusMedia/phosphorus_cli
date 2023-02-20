use std::time::Instant;

use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent,
};

use super::{event::UserEvent, AppMsg};

const LEFT_LABEL: usize = 0;
const RIGHT_LABEL: usize = 1;

/// Defines how many times ESC has to be
/// pressed before the application closes
const MAX_ESC_TOLERANCE: u16 = 2;

const STD_MSG: &'static str = "Press 2 times ESC to quit";
const QUIT_MSG: &'static str = "Press ESC again to quit";
/// Message shown when the help window is opened and visible
const HELP_MSG: &'static str = "Press ESC to close help window";
/// Message shown when a playlist view is opened and visible
const PLAYLIST_MSG: &'static str = "Press ESC to close playlist window";

const QUERY_SENT_MSG_1: &'static str = "Fetching results.  ";
const QUERY_SENT_MSG_2: &'static str = "Fetching results.. ";
const QUERY_SENT_MSG_3: &'static str = "Fetching results...";
const QUERY_SOLVED_MSG: &'static str = "Results fetched in";

#[derive(MockComponent)]
pub struct StatusBar {
    component: Container,
    is_secondary_window_active: bool,
    esc_count: u16,
    timer: Option<std::time::Instant>,
}

impl StatusBar {
    pub fn new() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Label::default().alignment(Alignment::Left).text(STD_MSG)),
            Box::new(Label::default().alignment(Alignment::Right).text("Welcome")),
        ];

        StatusBar {
            component: Container::default()
                .children(children)
                .background(Color::LightGreen)
                .foreground(Color::Black)
                .borders(Borders::default().sides(BorderSides::empty()))
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
            is_secondary_window_active: false,
            esc_count: 0,
            timer: None,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, UserEvent> for StatusBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();

        let event = match ev {
            Event::User(event) => event,
            Event::Tick => {
                if let Some(instant) = self.timer {
                    let child: &mut Box<dyn MockComponent> = children.get_mut(RIGHT_LABEL).unwrap();
                    let secs = instant.elapsed().as_secs();
                    match secs % 3 {
                        0 => {
                            child.attr(Attribute::Text, AttrValue::String(QUERY_SENT_MSG_1.into()))
                        }
                        1 => {
                            child.attr(Attribute::Text, AttrValue::String(QUERY_SENT_MSG_2.into()))
                        }
                        2 => {
                            child.attr(Attribute::Text, AttrValue::String(QUERY_SENT_MSG_3.into()))
                        }
                        _ => (),
                    }
                }

                return None;
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();

                if self.is_secondary_window_active {
                    self.is_secondary_window_active = false;
                    child.attr(Attribute::Text, AttrValue::String(STD_MSG.into()));
                    return Some(AppMsg::LoseFocus);
                }

                self.esc_count += 1;
                if self.esc_count == MAX_ESC_TOLERANCE {
                    return Some(AppMsg::Quit);
                }

                child.attr(Attribute::Text, AttrValue::String(QUIT_MSG.into()));
                return Some(AppMsg::None);
            }
            _ => {
                if !self.is_secondary_window_active {
                    let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();
                    child.attr(Attribute::Text, AttrValue::String(STD_MSG.into()));
                }
                self.esc_count = 0;
                return Some(AppMsg::None);
            }
        };

        match event {
            UserEvent::HelpOpened => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();
                child.attr(Attribute::Text, AttrValue::String(HELP_MSG.into()));
                self.is_secondary_window_active = true;
            }
            UserEvent::PlaylistViewOpened => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();
                child.attr(Attribute::Text, AttrValue::String(PLAYLIST_MSG.into()));
                self.is_secondary_window_active = true;
            }
            UserEvent::QuerySent => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(RIGHT_LABEL).unwrap();
                self.timer = Some(Instant::now());
                child.attr(Attribute::Text, AttrValue::String(QUERY_SENT_MSG_1.into()));
            }
            UserEvent::QueryResult(_) => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(RIGHT_LABEL).unwrap();
                if let Some(instant) = self.timer {
                    child.attr(
                        Attribute::Text,
                        AttrValue::String(format!(
                            "{} {:.3}s",
                            QUERY_SOLVED_MSG,
                            instant.elapsed().as_secs_f32()
                        )),
                    );
                    self.timer = None;
                }
            }
            UserEvent::PlaySong(song) => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(RIGHT_LABEL).unwrap();
                child.attr(
                    Attribute::Text,
                    AttrValue::String(format!(
                        "Playing {} by {}",
                        song.details().name(),
                        song.details().artist().unwrap_or("Uknown")
                    )),
                );
            }
            _ => (),
        }
        Some(AppMsg::None)
    }
}
