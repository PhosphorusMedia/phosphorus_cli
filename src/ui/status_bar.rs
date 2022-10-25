use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent,
};

use super::AppMsg;

const LEFT_LABEL: usize = 0;
const RIGHT_LABEL: usize = 1;

const STD_MSG: &'static str = "Press 2 times ESC to quit";
const QUIT_MSG: &'static str = "Press ESC again to quit";
const HELP_MSG: &'static str = "Press ESC to close help window";

#[derive(MockComponent)]
pub struct StatusBar {
    component: Container,
    secondary_window_active: bool,
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
            secondary_window_active: false,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, NoUserEvent> for StatusBar {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();

        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();
                child.attr(Attribute::Text, AttrValue::String(HELP_MSG.into()));
                self.secondary_window_active = true;
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();

                if self.secondary_window_active {
                    self.secondary_window_active = false;
                    child.attr(Attribute::Text, AttrValue::String(STD_MSG.into()));
                } else {
                    child.attr(Attribute::Text, AttrValue::String(QUIT_MSG.into()));
                }
            }
            Event::Tick => {}
            _ => {
                if !self.secondary_window_active {
                    let child: &mut Box<dyn MockComponent> = children.get_mut(LEFT_LABEL).unwrap();
                    child.attr(Attribute::Text, AttrValue::String(STD_MSG.into()));
                }
            }
        }
        Some(AppMsg::None)
    }
}
