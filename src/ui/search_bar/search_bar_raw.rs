use tui_realm_stdlib::Input;
use tuirealm::{
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Borders, Style, TextModifiers},
    AttrValue, Component, Event, MockComponent, NoUserEvent,
};

use super::AppMsg;

#[derive(MockComponent)]
pub(super) struct SearchBarRaw {
    component: Input,
}

impl SearchBarRaw {
    pub fn new() -> Self {
        let mut input = Input::default()
            .borders(Borders::default().sides(tuirealm::props::BorderSides::all()))
            .placeholder(
                "Search...",
                Style::default().add_modifier(TextModifiers::ITALIC),
            )
            .input_type(tuirealm::props::InputType::Text);

        input.attr(tuirealm::Attribute::Scroll, AttrValue::Flag(true));
        input.attr(tuirealm::Attribute::ScrollStep, AttrValue::Number(1));

        SearchBarRaw { component: input }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, NoUserEvent> for SearchBarRaw {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                ..
            }) => return Some(AppMsg::GoNextItem),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(AppMsg::LoseFocus),
            _ => CmdResult::None,
        };
        Some(AppMsg::None)
    }
}
