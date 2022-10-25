use core::playlist_manager::Playlist;

use tui_realm_stdlib::Table;
use tuirealm::{MockComponent, props::{TableBuilder, TextSpan, Color, TextModifiers}, Component, NoUserEvent, Event, event::{KeyEvent, Key}, command::{Cmd, Direction, Position}};

use crate::ui::AppMsg;

const UNKNOWN_ARTIST: &'static str = "Unkwnown";
const UNKNOWN_DURATION: &'static str = " - ";

#[derive(MockComponent)]
pub struct PlaylistWindow {
    component: Table
}

impl PlaylistWindow {
    pub fn new(playlist: &Playlist) -> Self {
        let songs = playlist.songs();

        let mut builder = TableBuilder::default();
        if songs.len() > 0 {
            for (index, item) in songs.iter().enumerate() {
                let details = item.details();
                builder.add_col(TextSpan::new(index.to_string()).italic());
                builder.add_col(TextSpan::new(details.name()).italic());
                builder.add_col(TextSpan::new(details.artist().unwrap_or(UNKNOWN_ARTIST)).italic());
                builder.add_col(TextSpan::new(details.duration_str().unwrap_or(UNKNOWN_DURATION.into())).italic());
                if index < songs.len() - 1 {
                    builder.add_row();
                }
            }
        }

        let mut component = Table::default()
            .highlighted_color(Color::LightYellow)
            .scroll(true)
            .title(playlist.name(), tuirealm::props::Alignment::Left)
            .headers(&["#", "Name", "Artist", "Duration"])
            .highlighted_str("➤ ")
            .row_height(1)
            .widths(&[5, 50, 35, 10])
            .modifiers(TextModifiers::BOLD | TextModifiers::UNDERLINED);

        if songs.len() > 0 {
            component = component.table(builder.build());
        }

        Self { component }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Component<AppMsg, NoUserEvent> for PlaylistWindow {
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