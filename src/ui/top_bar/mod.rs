use std::sync::mpsc::Sender;

use download_tracker::{DownloadTracker, TrackInfo};
use tui_realm_stdlib::{Container, Phantom};
use tuirealm::{
    command::{Cmd, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    Component, Event, MockComponent, State, StateValue,
};

mod download_tracker;
mod search_bar;

use self::search_bar::SearchBar;

use super::{event::UserEvent, AppMsg};

const SEARCH_BAR: usize = 1;
const DOWNLOAD_TRACKER: usize = 2;

#[derive(MockComponent)]
pub struct TopBar {
    component: Container,
    tx: Sender<TrackInfo>,
}

impl TopBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Default for TopBar {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Phantom::default()),
            SearchBar::default().boxed(),
            DownloadTracker::new(rx).boxed(),
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
                                Constraint::Percentage(60),
                                Constraint::Percentage(20),
                            ]
                            .as_ref(),
                        ),
                ),
            tx,
        }
    }
}

impl Component<AppMsg, UserEvent> for TopBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();

        let (child, cmd) = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => (SEARCH_BAR, Cmd::Move(tuirealm::command::Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => (SEARCH_BAR, Cmd::Move(tuirealm::command::Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => (SEARCH_BAR, Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                (SEARCH_BAR, Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => (SEARCH_BAR, Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => (SEARCH_BAR, Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => (SEARCH_BAR, Cmd::Type(ch)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(AppMsg::GoNextItem),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                let search_bar = children.get_mut(SEARCH_BAR).unwrap();
                if let State::One(StateValue::String(query)) = search_bar.state() {
                    return Some(AppMsg::QuerySent(query));
                }
                (SEARCH_BAR, Cmd::None)
            }
            Event::User(UserEvent::DownloadRegistered(song_name)) => {
                let _ = self.tx.send(TrackInfo::New(song_name));
                (DOWNLOAD_TRACKER, Cmd::Change)
            }
            _ => (SEARCH_BAR, Cmd::None),
        };

        let child = children.get_mut(child).unwrap();
        child.perform(cmd);

        Some(AppMsg::None)
    }
}
