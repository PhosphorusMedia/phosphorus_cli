use phosphorus_core::song::SongDetails;
use std::time::{Duration, Instant};

use tui_realm_stdlib::{Container, Label, Phantom, ProgressBar};
use tuirealm::{
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent,
};

use super::{event::UserEvent, AppMsg};

const LEFT_LABEL: usize = 1;
const CURRENT_TIME: usize = 2;
const PROGRESS_INDICATOR: usize = 4;
const LIMIT_TIME: usize = 6;

type Formatter = fn(&Duration) -> String;

/// Formatter for duration that produces a string in the
/// form of mm:ss.
fn short_formatter(duration: &Duration) -> String {
    let mut secs = duration.as_secs();
    let mins: u64 = secs / 60;
    secs -= mins * 60;
    format!("\n{:02}:{:02}", mins, secs)
}

/// Formatter for duration that produces a string in the
/// form of h:mm:ss.
fn long_formatter(duration: &Duration) -> String {
    let mut secs = duration.as_secs();
    let mut mins: u64 = secs / 60;
    secs -= mins * 60;
    let hours: u64 = mins / 60;
    mins = mins - hours * 60;
    format!("\n{}:{:02}:{:02}", hours, mins, secs)
}

#[derive(MockComponent)]
pub struct PlayerBar {
    component: Container,
    timing: Option<Instant>,
    formatter: Formatter,
}

impl PlayerBar {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn set_song(&mut self, details: &SongDetails) {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        children.get_mut(LEFT_LABEL).unwrap().attr(
            Attribute::Text,
            AttrValue::String(format!(
                "\n{} | {}",
                details.name(),
                details.artist().unwrap_or("Unknown")
            )),
        );

        // Initialize a timer for tracking reproduction state
        self.timing = Some(Instant::now());

        // Select the appropriate formatter function according to
        // song duration
        self.formatter = short_formatter;
        if let Some(duration) = details.duration() {
            // If the song lasts for at least one hour, the
            // long formatter is used
            if duration.as_secs() >= 3600 {
                self.formatter = long_formatter;
            }
            children.get_mut(LIMIT_TIME).unwrap().attr(
                Attribute::Text,
                AttrValue::String((self.formatter)(duration)),
            );
        } else {
            children
                .get_mut(LIMIT_TIME)
                .unwrap()
                .attr(Attribute::Text, AttrValue::String("\n--:--".into()));
        }

        children.get_mut(CURRENT_TIME).unwrap().attr(
            Attribute::Text,
            AttrValue::String((self.formatter)(&self.timing.unwrap().elapsed())),
        );
        children.get_mut(PROGRESS_INDICATOR).unwrap().attr(
            Attribute::Value,
            AttrValue::Payload(tuirealm::props::PropPayload::One(
                tuirealm::props::PropValue::F64(0.0),
            )),
        );
    }
}

impl Default for PlayerBar {
    fn default() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            Box::new(Phantom::default()),
            Box::new(
                Label::default()
                    .alignment(Alignment::Left)
                    .text("\nNo song | Unknown"),
            ),
            Box::new(Label::default().alignment(Alignment::Right).text("\n--:--")),
            Box::new(Phantom::default()),
            Box::new(
                ProgressBar::default()
                    .progress(0.0)
                    .borders(Borders::default().sides(BorderSides::BOTTOM))
                    .foreground(Color::LightYellow),
            ),
            Box::new(Phantom::default()),
            Box::new(Label::default().alignment(Alignment::Left).text("\n--:--")),
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
                            [
                                Constraint::Length(1),      // Space between border and content
                                Constraint::Percentage(20), // Song name and artis
                                Constraint::Percentage(5),  // Current timing
                                Constraint::Length(1), // Space between current timing and progress
                                Constraint::Percentage(50), // Progress indicator
                                Constraint::Length(1), // Space between progress and duration
                                Constraint::Percentage(5), // Duration
                                Constraint::Percentage(20), // Empty space on the right
                            ]
                            .as_ref(),
                        ),
                ),
            timing: None,
            formatter: short_formatter,
        }
    }
}

impl Component<AppMsg, UserEvent> for PlayerBar {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();

        match ev {
            Event::User(UserEvent::PlaySong(song)) => {
                self.set_song(song.details());
            }
            Event::Tick => {
                if let Some(timer) = self.timing {
                    let duration = timer.elapsed();
                    if duration.as_secs() > 0 {
                        children.get_mut(CURRENT_TIME).unwrap().attr(
                            Attribute::Text,
                            AttrValue::String((self.formatter)(&duration)),
                        );
                    }
                }
            }
            _ => {}
        }

        None
    }
}
