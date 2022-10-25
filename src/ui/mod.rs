use core::{playlist_manager::PlaylistManager, queue::QueueManager};
use std::time::Duration;

use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, EventListenerCfg, NoUserEvent, Sub, SubEventClause, Update,
};

use crate::ui::{app_window::AppWindow, search_bar::SearchBar, status_bar::StatusBar};

mod app_window;
mod secondary_window;
mod playlist_list;
mod queue;
mod search_bar;
mod status_bar;
mod welcome_window;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Label,
    AppWindow,
    SearchBar,
    StatusBar,
}

#[derive(Debug, PartialEq)]
pub enum AppMsg {
    /// Closes the application
    Quit,
    /// The current active componen looses its focus
    LoseFocus,
    /// The focus is passed to the next component
    GoNextItem,
    /// Is like calling GoNext n time, so GoNextItem
    /// is equivalent to GoForward(1)
    GoForward(u16),
    ShowHelp,
    ShowPlaylist,
    None,
}

pub enum FocusableItem {
    SearchBar,
    PlaylistList,
    Queue,
    MainWindow,
    SecondaryWindow,
}

impl FocusableItem {
    pub fn next(&self) -> Self {
        match self {
            FocusableItem::SearchBar => FocusableItem::PlaylistList,
            FocusableItem::PlaylistList => FocusableItem::MainWindow,
            FocusableItem::Queue => FocusableItem::SearchBar,
            FocusableItem::MainWindow => FocusableItem::Queue,
            FocusableItem::SecondaryWindow => FocusableItem::Queue,
        }
    }

    pub fn to_id(&self) -> Id {
        match self {
            FocusableItem::SearchBar => Id::SearchBar,
            _ => Id::AppWindow,
        }
    }

    pub fn below_item(&self) -> Option<Self> {
        match self {
            FocusableItem::SecondaryWindow => Some(FocusableItem::MainWindow),
            _ => None,
        }
    }
}

pub struct Model {
    /// Application
    pub app: Application<Id, AppMsg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
    // Used to track the active component
    active: FocusableItem,
    secondary_window_active: bool,
    esc_count: u8,
}

impl Model {
    pub fn new(playlist_manager: PlaylistManager, queue_manager: QueueManager) -> Self {
        Self {
            app: Self::init_app(playlist_manager, queue_manager),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            active: FocusableItem::SearchBar,
            secondary_window_active: false,
            esc_count: 0,
        }
    }

    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // SearchBar
                            Constraint::Min(6),    // AppWindow
                            Constraint::Length(1), // StatusBar
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::SearchBar, f, chunks[0]);
                self.app.view(&Id::AppWindow, f, chunks[1]);
                self.app.view(&Id::StatusBar, f, chunks[2]);
            })
            .is_ok());
    }

    pub fn init_app(
        playlist_manager: PlaylistManager,
        queue_manager: QueueManager,
    ) -> Application<Id, AppMsg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock
        let mut app: Application<Id, AppMsg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(65)),
        );

        // Mounts the components
        assert!(app
            .mount(Id::SearchBar, SearchBar::default().boxed(), Vec::default())
            .is_ok());
        assert!(app
            .mount(
                Id::AppWindow,
                Box::new(AppWindow::new(playlist_manager, queue_manager)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(Id::StatusBar, StatusBar::new().boxed(), Vec::default())
            .is_ok());

        assert!(app
            .subscribe(
                &Id::AppWindow,
                Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Char('h'),
                        modifiers: KeyModifiers::CONTROL
                    }),
                    tuirealm::SubClause::Always
                )
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::StatusBar,
                Sub::new(SubEventClause::Any, tuirealm::SubClause::Always)
            )
            .is_ok());

        // Initializes focus
        assert!(app.active(&Id::SearchBar).is_ok());

        app
    }
}

impl Update<AppMsg> for Model {
    fn update(&mut self, msg: Option<AppMsg>) -> Option<AppMsg> {
        let mut esc_pressed = false;

        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                AppMsg::Quit => self.quit = true,
                AppMsg::LoseFocus => {
                    if self.secondary_window_active {
                        self.secondary_window_active = false;
                        if let Some(item) = self.active.below_item() {
                            self.active = item;
                            assert!(self.app.active(&self.active.to_id()).is_ok());
                        }
                    } else {
                        esc_pressed = true;
                        self.esc_count += 1;
                        if self.esc_count == 2 {
                            return Some(AppMsg::Quit);
                        };
                    }
                }
                AppMsg::GoNextItem => {
                    self.active = self.active.next();
                    /* This if statement is necessary to display components
                      into the container as not focused when another component
                      of the same container is focused.

                      DO NOT REMOVE
                    */
                    if let FocusableItem::MainWindow = self.active {
                        if self.secondary_window_active {
                            self.active = FocusableItem::SecondaryWindow;
                        }
                    } else if let FocusableItem::PlaylistList = self.active {
                        assert!(self.app.active(&self.active.to_id()).is_ok());
                    }
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::GoForward(mut n) => {
                    while n > 0 {
                        self.active = self.active.next();
                        n -= 1;
                    }
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::ShowHelp => {
                    self.active = FocusableItem::SecondaryWindow;
                    self.secondary_window_active = true;

                    // Again, I don't know why this has to repeted
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::ShowPlaylist => {
                    self.active = FocusableItem::SecondaryWindow;
                    self.secondary_window_active = true;

                    // Again, I don't know why this has to repeted
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::None => {
                    esc_pressed = true;
                }
            }
        }

        if !esc_pressed {
            self.esc_count = 0;
        }

        None
    }
}
