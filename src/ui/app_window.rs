use core::playlist_manager::PlaylistManager;

use tui_realm_stdlib::Container;
use tuirealm::{
    command::{Cmd, Direction as CDir, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{BorderSides, Borders, Layout, Style},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent,
};

use super::{playlist_list::PlaylistList, queue::Queue, welcome_window::WelcomWindow, AppMsg, help_window::HelpWindow};

const PLAYLIST_LIST: usize = 0;
const MAIN_WINDOW: usize = 1;
const QUEUE: usize = 2;

#[derive(PartialEq, Clone, Copy)]
pub enum MainWindowType {
    Welcome,
    Help,
}

impl MainWindowType {
    pub fn need_focus(&self) -> bool {
        match self {
            MainWindowType::Welcome => false,
            MainWindowType::Help => true
        }
    }

    pub fn get_component(&self) -> Box<dyn MockComponent> {
        match self {
            MainWindowType::Welcome => WelcomWindow::default().boxed(),
            MainWindowType::Help => HelpWindow::default().boxed(),
        }
    }
}

#[derive(MockComponent)]
pub struct AppWindow {
    component: Container,
    active: Option<usize>,
    main_window_type: MainWindowType,
    previous_window: Option<MainWindowType>,
    playlist_manager: PlaylistManager,
}

impl AppWindow {
    pub fn new(pm: PlaylistManager) -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            PlaylistList::default()
                .list(pm.names().iter().map(|name| String::from(*name)).collect())
                .boxed(),
            WelcomWindow::default().boxed(),
            Queue::default()
                .list(vec![
                    String::from("Song 1"),
                    String::from("Song 2"),
                    String::from("Song 3"),
                ])
                .boxed(),
        ];

        AppWindow {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::empty()))
                .children(children)
                .layout(
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
            active: Some(PLAYLIST_LIST),
            main_window_type: MainWindowType::Welcome,
            previous_window: None,
            playlist_manager: pm,
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
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL
            }) => {
                if self.main_window_type != MainWindowType::Help {
                    self.previous_window = Some(self.main_window_type);
                    self.main_window_type = MainWindowType::Help;
                    self.component.children.remove(MAIN_WINDOW);
                    self.component.children.insert(MAIN_WINDOW,  self.main_window_type.get_component());
                    self.active = Some(MAIN_WINDOW);
                    return Some(AppMsg::ShowHelp);
                }
            }
            _ => (),
        };

        let index = self.active.unwrap();
        let children: &mut Vec<Box<dyn MockComponent>> = self.component.children.as_mut();
        let mut child: &mut Box<dyn MockComponent> = children.get_mut(index).unwrap();
        child.attr(Attribute::Focus, AttrValue::Flag(true));

        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                if self.main_window_type == MainWindowType::Help {
                    self.main_window_type = self.previous_window.unwrap_or(MainWindowType::Welcome);
                    self.previous_window = None;
                    self.component.children.remove(MAIN_WINDOW);
                    self.component.children.insert(MAIN_WINDOW,  self.main_window_type.get_component());
                }
                return Some(AppMsg::LoseFocus)
            },
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                let mut msg = AppMsg::GoNextItem;

                if index < 2 {
                    child.attr(Attribute::Focus, AttrValue::Flag(false));

                    self.active = Some(index + 1);
                    if index == PLAYLIST_LIST && !self.main_window_type.need_focus() {
                        self.active = Some(index + 2);
                        msg = AppMsg::GoForward(2);
                    }
                    child = children.get_mut(self.active.unwrap()).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                } else {
                    child.attr(Attribute::Focus, AttrValue::Flag(false));
                    child = children.get_mut(PLAYLIST_LIST).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                    self.active = Some(PLAYLIST_LIST);
                }
                return Some(msg);
            }
            _ => (),
        };

        if index == PLAYLIST_LIST || index == QUEUE {
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
            if self.main_window_type == MainWindowType::Help {
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
            }
        }

        Some(AppMsg::None)
    }
}
