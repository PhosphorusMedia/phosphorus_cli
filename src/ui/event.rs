use core::song::Song;
use std::sync::mpsc::Receiver;

use plugin_manager::query::QueryResult;
use tuirealm::{listener::Poll, Event};

#[derive(Clone, PartialOrd, Debug)]
pub enum UserEvent {
    /// The help window has been opened
    HelpOpened,
    /// A playlist view has been opened
    PlaylistViewOpened,
    /// A secondary windows has been closed (`ESC` has been pressed)
    SecondaryWindowClosed,
    /// Sent a query to the `plugin_manager`
    QuerySent,
    /// A query has produced a successfull result
    QueryResult(QueryResult),
    /// A query has failed and produces and error
    QueryError(String),
    /// Started playing a song
    PlaySong(Song),
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::QueryResult(_), _other) => true,
            (Self::QueryError(_), _other) => true,
            (Self::PlaySong(_), _other) => true,
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}

impl Eq for UserEvent {}

pub struct UserEventPort {
    rx: Receiver<UserEvent>,
}

/// Receives events from a `mpsc` channel and triggers them
impl UserEventPort {
    pub fn new(rx: Receiver<UserEvent>) -> Self {
        Self { rx }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Poll<UserEvent> for UserEventPort {
    fn poll(&mut self) -> tuirealm::listener::ListenerResult<Option<tuirealm::Event<UserEvent>>> {
        let event = self.rx.try_recv();
        if event.is_err() {
            return Ok(None);
        }
        let event = event.unwrap();
        Ok(Some(Event::User(event)))
    }
}
