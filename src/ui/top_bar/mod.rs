use std::sync::mpsc::{Receiver, Sender};

use download_tracker::{DownloadTracker, InternalTrackInfo};
use phosphorus_core::TrackInfo;
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

use super::{AppMsg, UserEvent};

const SEARCH_BAR: usize = 1;
const DOWNLOAD_TRACKER: usize = 2;

#[derive(MockComponent)]
pub struct TopBar {
    component: Container,
    internal_tx: Sender<InternalTrackInfo>,
    external_rx: Receiver<TrackInfo>,
}

impl TopBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn new(download_track_rx: Receiver<TrackInfo>) -> Self {
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
            internal_tx: tx,
            external_rx: download_track_rx,
        }
    }
}

impl Component<AppMsg, UserEvent> for TopBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        let msg = AppMsg::None;

        let (child, cmd, msg) = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => (
                SEARCH_BAR,
                Cmd::Move(tuirealm::command::Direction::Left),
                msg,
            ),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => (
                SEARCH_BAR,
                Cmd::Move(tuirealm::command::Direction::Right),
                msg,
            ),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => (SEARCH_BAR, Cmd::GoTo(Position::Begin), msg),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                (SEARCH_BAR, Cmd::GoTo(Position::End), msg)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => (SEARCH_BAR, Cmd::Cancel, msg),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => (SEARCH_BAR, Cmd::Delete, msg),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => (SEARCH_BAR, Cmd::Type(ch), msg),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                (SEARCH_BAR, Cmd::None, AppMsg::GoNextItem)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                let search_bar = children.get_mut(SEARCH_BAR).unwrap();
                if let State::One(StateValue::String(query)) = search_bar.state() {
                    (SEARCH_BAR, Cmd::None, AppMsg::QuerySent(query))
                } else {
                    (SEARCH_BAR, Cmd::None, msg)
                }
            }
            Event::User(UserEvent::DownloadRegistered(song)) => {
                let _ = self
                    .internal_tx
                    .send(InternalTrackInfo::New(song.details().name().to_string()));
                (DOWNLOAD_TRACKER, Cmd::Change, msg)
            }
            Event::Tick => {
                let track_msg = self.external_rx.try_recv();
                if let Err(_) = track_msg {
                    (DOWNLOAD_TRACKER, Cmd::None, msg)
                } else {
                    let track_msg = track_msg.unwrap();
                    let msg = match track_msg {
                        TrackInfo::New(song) => {
                            let _ = self
                                .internal_tx
                                .send(InternalTrackInfo::New(song.details().name().to_string()));
                            msg
                        }
                        TrackInfo::Started(_) => {
                            let _ = self.internal_tx.send(InternalTrackInfo::Started);
                            msg
                        }
                        TrackInfo::Progress(_, perc) => {
                            let _ = self.internal_tx.send(InternalTrackInfo::Progress(perc));
                            msg
                        }
                        TrackInfo::Finished(song) => {
                            let _ = self.internal_tx.send(InternalTrackInfo::Finished);
                            AppMsg::DownloadFinished(song)
                        }
                        TrackInfo::Failed(song, err) => {
                            let _ = self.internal_tx.send(InternalTrackInfo::Failed(err.clone()));
                            AppMsg::DownloadFailed(song, err)
                        }
                    };
                    (DOWNLOAD_TRACKER, Cmd::Change, msg)
                }
            }
            _ => (SEARCH_BAR, Cmd::None, msg),
        };

        let child = children.get_mut(child).unwrap();
        child.perform(cmd);

        Some(msg)
    }
}
