use std::sync::mpsc::{Receiver, Sender};

use tui_realm_stdlib::{Container, Label, Phantom};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, BorderSides, Borders, Color, Layout, TextModifiers},
    tui::layout::{Constraint, Direction},
    MockComponent, State,
};

const DOWNLOAD_FOLLOWER: usize = 1;

pub enum InternalTrackInfo {
    New(String),
    Started,
    Progress(f32),
    Finished,
    Failed(String),
}

pub(super) struct DownloadTracker {
    component: Container,
    downloads: Vec<String>,
    track_info_rx: Receiver<InternalTrackInfo>,
    download_count_tx: Sender<usize>,
    current: Option<String>,
}

impl DownloadTracker {
    pub fn new(rx: Receiver<InternalTrackInfo>) -> Self {
        let (tx, internal_rx) = std::sync::mpsc::channel();

        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Phantom::default()),
            Box::new(
                Label::default()
                    .modifiers(TextModifiers::ITALIC)
                    .alignment(Alignment::Right),
            ),
            DownloadCounter::new(internal_rx).boxed(),
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
                                Constraint::Max(1),
                                Constraint::Min(DOWNLOAD_STR_LENGTH as u16),
                                Constraint::Min(4),
                            ]
                            .as_ref(),
                        )
                        .horizontal_margin(1),
                ),
            downloads: vec![],
            track_info_rx: rx,
            download_count_tx: tx,
            current: None,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl MockComponent for DownloadTracker {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.component.attr(attr, value);
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            tuirealm::command::Cmd::Change => {
                match self.track_info_rx.recv().unwrap() {
                    InternalTrackInfo::New(song_name) => {
                        self.downloads.push(song_name);
                        let _ = self.download_count_tx.send(self.downloads.len());
                        self.component.perform(Cmd::Change);
                    }
                    InternalTrackInfo::Started => {
                        let song_name =self.downloads.pop().unwrap();
                        let _ = self.download_count_tx.send(self.downloads.len());
                        self.component.perform(Cmd::Change);
                        let follower = self.component.children.get_mut(DOWNLOAD_FOLLOWER).unwrap();
                        follower.attr(
                            tuirealm::Attribute::Text,
                            tuirealm::AttrValue::String(format_downloading(&song_name, 0.0)),
                        );
                        self.current = Some(song_name);
                    }
                    InternalTrackInfo::Progress(perc) => {
                        let song_name = self.current.as_ref().unwrap();
                        let follower = self.component.children.get_mut(DOWNLOAD_FOLLOWER).unwrap();
                        follower.attr(
                            tuirealm::Attribute::Text,
                            tuirealm::AttrValue::String(format_downloading(&song_name, perc)),
                        );
                    }
                    InternalTrackInfo::Finished => {
                        self.current = None;
                        let follower = self.component.children.get_mut(DOWNLOAD_FOLLOWER).unwrap();
                        follower.attr(
                            tuirealm::Attribute::Text,
                            tuirealm::AttrValue::String(String::new()),
                        );
                    }
                    InternalTrackInfo::Failed(err) => {
                        self.current = None;
                        let follower = self.component.children.get_mut(DOWNLOAD_FOLLOWER).unwrap();
                        follower.attr(
                            tuirealm::Attribute::Text,
                            tuirealm::AttrValue::String(format!("\n{} ", err)),
                        );
                    }
                };
                CmdResult::None
            }
            _ => self.component.perform(cmd),
        }
    }
}

struct DownloadCounter {
    component: Container,
    rx: Receiver<usize>,
}

impl DownloadCounter {
    pub fn new(rx: Receiver<usize>) -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Phantom::default()),
            Box::new(
                Label::default()
                    .alignment(Alignment::Center)
                    .text(format_count(0)),
            ),
            Box::new(Phantom::default()),
        ];

        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .foreground(Color::Reset)
                .children(children)
                .layout(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Min(1), Constraint::Fill(1), Constraint::Min(1)].as_ref(),
                        ),
                ),
            rx,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl MockComponent for DownloadCounter {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.component.attr(attr, value);
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            tuirealm::command::Cmd::Change => {
                let count = match self.rx.recv() {
                    Ok(count) => count,
                    Err(err) => {
                        eprintln!("ERROR: {err}");
                        std::process::exit(1);
                    }
                };
                self.component.attr(
                    tuirealm::Attribute::Text,
                    tuirealm::AttrValue::String(format_count(count)),
                );
                tuirealm::command::CmdResult::None
            }
            _ => self.component.perform(cmd),
        }
    }
}

const MAX_NAME_LENGTH: usize = 8;
const DOWNLOAD_STR_LENGTH: usize = MAX_NAME_LENGTH + 12;
fn format_downloading(name: &str, percentage: f32) -> String {
    if name.len() > MAX_NAME_LENGTH + 2 {
        format!(
            "\n{:<MAX_NAME_LENGTH$}..: {:02.2}% ",
            &name[0..MAX_NAME_LENGTH],
            percentage
        )
    } else {
        format!("\n{:<MAX_NAME_LENGTH$}: {:02.2}% ", name, percentage)
    }
}

fn format_count(count: usize) -> String {
    format!("\n{:<02}", count)
}
