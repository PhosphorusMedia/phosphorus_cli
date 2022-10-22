use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent},
    props::{Color, TableBuilder, TextSpan},
    Component, Event, MockComponent, NoUserEvent,
};

use super::AppMsg;

#[derive(MockComponent)]
pub struct PlaylistList {
    component: Table,
    list: Vec<String>,
}

impl PlaylistList {
    fn new(mut list: Option<Vec<String>>) -> Self {
        if list.is_none() {
            list = Some(vec![]);
        }

        let list = list.unwrap();
        let mut builder = TableBuilder::default();
        if list.len() > 0 {
            for item in &list.as_slice()[0..&list.len() - 1] {
                builder.add_col(TextSpan::new(item));
                builder.add_row();
            }
            builder.add_col(TextSpan::new(&list.get(&list.len() - 1).unwrap()));
        }

        let component = Table::default()
            .highlighted_color(Color::LightYellow)
            .scroll(true)
            .table(builder.build())
            .headers(&["Playlists"])
            .highlighted_str("➤ ")
            .row_height(1)
            .widths(&[100]);

        Self { component, list }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn list(self, list: Vec<String>) -> Self {
        Self::new(Some(list))
    }
}

impl Default for PlaylistList {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Component<AppMsg, NoUserEvent> for PlaylistList {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => Cmd::Scroll(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => Cmd::Scroll(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => Cmd::GoTo(Position::Begin),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(AppMsg::GoNextItem),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(AppMsg::LoseFocus),
            _ => Cmd::None,
        };

        let _ = self.perform(cmd);
        Some(AppMsg::None)
    }
}
