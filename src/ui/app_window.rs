use tui_realm_stdlib::Container;
use tuirealm::{
    command::{Cmd, Direction as CDir, Position},
    event::{Key, KeyEvent},
    props::{Layout, Style},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent,
};

use super::{playlist_list::PlaylistList, queue::Queue, search_bar::SearchBar, AppMsg};

#[derive(MockComponent)]
pub struct AppWindow {
    component: Container,
    active: Option<usize>,
}

impl AppWindow {
    pub fn new() -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            PlaylistList::default()
                .list(vec![
                    String::from("Playlist 1"),
                    String::from("Playlist 2"),
                    String::from("Playlist 3"),
                ])
                .boxed(),
            SearchBar::default().boxed(),
            Queue::default()
                .list(vec![
                    String::from("Song 1"),
                    String::from("Song 2"),
                    String::from("Song 3"),
                ])
                .boxed(),
        ];

        AppWindow {
            component: Container::default().children(children).layout(
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(20), // LeftBar
                            Constraint::Percentage(60), // MainWindow
                            Constraint::Percentage(20), // RightBar
                        ]
                        .as_ref(),
                    ),
            ),
            active: Some(0),
        }
    }
}

impl Component<AppMsg, NoUserEvent> for AppWindow {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<AppMsg> {        
        match ev {
            Event::FocusGained => {
                for child in self.component.children.as_mut_slice() {
                    child.attr(Attribute::Focus, AttrValue::Flag(false));
                    child.attr(Attribute::FocusStyle, AttrValue::Style(Style::default()));
                }
            },
            _ => ()
        };

        let index = self.active.unwrap();
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        let mut child: &mut Box<dyn MockComponent> = children.get_mut(index).unwrap();
        child.attr(Attribute::Focus, AttrValue::Flag(true));

        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(AppMsg::LoseFocus)
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, ..}) => {
                if index < 2 {
                    self.active = Some(index + 1);
                    child.attr(Attribute::Focus, AttrValue::Flag(false));
                    child = children.get_mut(index).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                } else {
                    child.attr(Attribute::Focus, AttrValue::Flag(false));
                    child = children.get_mut(0).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                    self.active = Some(0);
                }
                return Some(AppMsg::GoNextItem);
            }
            _ => (),
        };

        if index == 0 || index == 2 {
            let cmd = match ev {
                Event::Keyboard(KeyEvent {
                    code: Key::Down, ..
                }) => Cmd::Move(CDir::Down),
                Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Cmd::Move(CDir::Up),
                Event::Keyboard(KeyEvent {
                    code: Key::PageDown,
                    ..
                }) => Cmd::Scroll(CDir::Down),
                Event::Keyboard(KeyEvent {
                    code: Key::PageUp, ..
                }) => Cmd::Scroll(CDir::Up),
                Event::Keyboard(KeyEvent {
                    code: Key::Home, ..
                }) => Cmd::GoTo(Position::Begin),
                Event::Keyboard(KeyEvent { code: Key::End, .. }) => Cmd::GoTo(Position::End),
                Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(AppMsg::LoseFocus),
                _ => Cmd::None,
            };

            let _ = child.perform(cmd);
        } else {
        }

        Some(AppMsg::None)
    }
}
