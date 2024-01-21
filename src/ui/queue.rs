use phosphorus_core::song::SongDetails;

use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent},
    props::{Color, TableBuilder, TextModifiers, TextSpan},
    Component, Event, MockComponent,
};

use super::{event::UserEvent, AppMsg};

#[derive(MockComponent)]
pub struct Queue {
    component: Table,
}

impl Queue {
    fn new(mut list: Option<Vec<&SongDetails>>) -> Self {
        if list.is_none() {
            list = Some(vec![]);
        }

        let list = list.unwrap();
        let mut builder = TableBuilder::default();
        if list.len() > 0 {
            for item in &list.as_slice()[0..&list.len() - 1] {
                builder.add_col(TextSpan::new(item.name()).italic());
                builder
                    .add_col(TextSpan::new(item.duration_str().unwrap_or(" - ".into())).italic());
                builder.add_row();
            }
            let item = list.get(list.len() - 1).unwrap();
            builder.add_col(TextSpan::new(item.name()).italic());
            builder.add_col(TextSpan::new(item.duration_str().unwrap_or(" - ".into())).italic());
        }

        let mut component = Table::default()
            .highlighted_color(Color::LightYellow)
            .scroll(true)
            .headers(&["Reproduction queue"])
            .highlighted_str("➤ ")
            .row_height(1)
            .widths(&[70, 30])
            .modifiers(TextModifiers::BOLD | TextModifiers::UNDERLINED);

        if list.len() > 0 {
            component = component.table(builder.build());
        }

        Self { component }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn list(self, list: Vec<&SongDetails>) -> Self {
        Self::new(Some(list))
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Component<AppMsg, UserEvent> for Queue {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
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
            _ => Cmd::None,
        };

        let _ = self.perform(cmd);
        Some(AppMsg::None)
    }
}
