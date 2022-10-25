use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent, event::{KeyEvent, Key},
};

use super::{event::UserEvent, AppMsg};

const LEFT_LABEL: usize = 0;
const RIGHT_LABEL: usize = 1;

const MAX_ESC_TOLERANCE: u16 = 2;

const STD_MSG: &'static str = "Press 2 times ESC to quit";
const QUIT_MSG: &'static str = "Press ESC again to quit";
const HELP_MSG: &'static str = "Press ESC to close help window";
const PLAYLIST_MSG: &'static str = "Press ESC to close playlist window";

#[derive(MockComponent)]
pub struct StatusBar {
    component: Container,
    is_secondary_window_active: bool,
    esc_count: u16,
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
            esc_count: 0
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
            Event::Tick => return Some(AppMsg::None),
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                ..
            }) => {
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
                return Some(AppMsg::None)
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
        }
        Some(AppMsg::None)
    }
}
