use core::{playlist_manager::PlaylistManager, queue::QueueManager};

use tui_realm_stdlib::Container;
use tuirealm::{
    command::{Cmd, Direction as CDir, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{BorderSides, Borders, Layout},
    tui::layout::{Constraint, Direction},
    AttrValue, Attribute, Component, Event, MockComponent, State, StateValue,
};

use super::{
    event::UserEvent,
    playlist_list::PlaylistList,
    queue::Queue,
    secondary_window::{HelpWindow, PlaylistWindow},
    welcome_window::WelcomWindow,
    AppMsg,
};

const PLAYLIST_LIST: usize = 0;
const MAIN_WINDOW: usize = 1;
const QUEUE: usize = 2;

#[derive(PartialEq, Clone, Copy)]
pub enum MainWindowType {
    Welcome,
    Help,
    PlaylistSongs,
}

impl MainWindowType {
    pub fn need_focus(&self) -> bool {
        match self {
            MainWindowType::Welcome => false,
            MainWindowType::Help => true,
            MainWindowType::PlaylistSongs => true,
        }
    }

    pub fn is_secondary(&self) -> bool {
        match self {
            MainWindowType::Welcome => false,
            MainWindowType::Help => true,
            MainWindowType::PlaylistSongs => true,
        }
    }

    pub fn default(&self) -> Option<Box<dyn MockComponent>> {
        match self {
            MainWindowType::Welcome => Some(WelcomWindow::default().boxed()),
            MainWindowType::Help => Some(HelpWindow::default().boxed()),
            _ => None,
        }
    }

    pub fn is_table_like(&self) -> bool {
        match self {
            MainWindowType::Welcome => false,
            MainWindowType::Help => true,
            MainWindowType::PlaylistSongs => true,
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
    queue_manager: QueueManager,
}

impl AppWindow {
    pub fn new(playlist_manager: PlaylistManager, queue_manager: QueueManager) -> Self {
        let children: Vec<Box<dyn MockComponent>> = vec![
            PlaylistList::default()
                .list(
                    playlist_manager
                        .names()
                        .iter()
                        .map(|name| String::from(*name))
                        .collect(),
                )
                .boxed(),
            WelcomWindow::default().boxed(),
            Queue::default().list(queue_manager.pending()).boxed(),
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
            playlist_manager,
            queue_manager,
        }
    }
}

impl Component<AppMsg, UserEvent> for AppWindow {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<AppMsg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                if self.main_window_type != MainWindowType::Help {
                    if self.main_window_type.is_secondary() {
                        self.previous_window = Some(MainWindowType::Welcome);
                    } else {
                        self.previous_window = Some(self.main_window_type);
                    }
                    self.main_window_type = MainWindowType::Help;
                    self.component.children.remove(MAIN_WINDOW);
                    self.component
                        .children
                        .insert(MAIN_WINDOW, self.main_window_type.default().unwrap());
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
                if self.main_window_type.is_secondary() {
                    self.main_window_type = self.previous_window.unwrap_or(MainWindowType::Welcome);
                    self.previous_window = None;
                    children.remove(MAIN_WINDOW);
                    children.insert(MAIN_WINDOW, self.main_window_type.default().unwrap());
                }
                return Some(AppMsg::None);
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                // Removes focus from current active component
                child.attr(Attribute::Focus, AttrValue::Flag(false));
                let mut step = 1;
                let mut found_focusable = false;
                while index + step <= 2 && !found_focusable {
                    match index + step {
                        MAIN_WINDOW => {
                            if self.main_window_type.need_focus() {
                                found_focusable = true;
                            } else {
                                step += 1;
                            }
                        }
                        QUEUE => {
                            if !self.queue_manager.is_empty() {
                                found_focusable = true;
                            } else {
                                step += 1;
                            }
                        }
                        _ => (),
                    }
                }
                if index + step <= 2 {
                    self.active = Some(index + step);
                    //msg = AppMsg::GoForward(step as u16);
                    child = children.get_mut(self.active.unwrap()).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                } else {
                    child = children.get_mut(PLAYLIST_LIST).unwrap();
                    child.attr(Attribute::Focus, AttrValue::Flag(true));
                    self.active = Some(PLAYLIST_LIST);
                }
                return Some(AppMsg::GoForward(step as u16));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if let Some(PLAYLIST_LIST) = self.active {
                    if let State::One(StateValue::Usize(index)) = child.state() {
                        let playlist = self.playlist_manager.playlists().get(index).unwrap();
                        if self.main_window_type.is_secondary() {
                            self.previous_window = Some(MainWindowType::Welcome);
                        } else {
                            self.previous_window = Some(self.main_window_type);
                        }
                        self.main_window_type = MainWindowType::PlaylistSongs;
                        children.remove(MAIN_WINDOW);
                        children.insert(MAIN_WINDOW, PlaylistWindow::new(playlist).boxed());
                        self.active = Some(MAIN_WINDOW);
                        return Some(AppMsg::ShowPlaylist);
                    }
                }
            }
            _ => (),
        };

        if index == PLAYLIST_LIST || index == QUEUE || self.main_window_type.is_table_like() {
            return table_events(&ev, child);
        }

        Some(AppMsg::None)
    }
}

fn table_events(
    ev: &tuirealm::Event<UserEvent>,
    child: &mut Box<dyn MockComponent>,
) -> Option<AppMsg> {
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
        _ => Cmd::None,
    };

    child.perform(cmd);
    Some(AppMsg::None)
}
