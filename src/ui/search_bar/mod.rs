use tui_realm_stdlib::{Container, Phantom};
use tuirealm::{
    command::{Cmd, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{self, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    Component, Event, MockComponent,
};

mod search_bar_raw;

use self::search_bar_raw::SearchBarRaw;

use super::{event::UserEvent, AppMsg};

#[derive(MockComponent)]
pub struct SearchBar {
    component: Container,
}

impl SearchBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Phantom::default()),
            SearchBarRaw::new().boxed(),
            Box::new(Phantom::default()),
        ];

        Self {
            component: Container::default()
                .borders(Borders::default().sides(props::BorderSides::empty()))
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

impl Component<AppMsg, UserEvent> for SearchBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        let child: &mut Box<dyn MockComponent> = children.get_mut(1).unwrap();

        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Cmd::Move(tuirealm::command::Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Cmd::Move(tuirealm::command::Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => Cmd::Cancel,
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Cmd::Delete,
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => Cmd::Type(ch),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(AppMsg::GoNextItem),
            _ => Cmd::None,
        };

        let _ = child.perform(cmd);
        Some(AppMsg::None)
    }
}
