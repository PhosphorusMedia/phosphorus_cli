use std::time::Duration;

use tuirealm::{NoUserEvent, Application, terminal::TerminalBridge, tui::layout::{Layout, Direction, Constraint}, Update, EventListenerCfg};

use crate::ui::app_window::AppWindow;

mod app_window;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    AppWindow,
    Label
}

#[derive(Debug, PartialEq)]
pub enum AppMsg {
    Quit
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
}

impl Model {
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
                            Constraint::Length(25), // AppWindow
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::AppWindow, f, chunks[0]);
            })
            .is_ok());
    }

    pub fn init_app() -> Application<Id, AppMsg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock
        let mut app: Application<Id, AppMsg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
    
        // Mounts the components
        assert!(app
            .mount(
                Id::AppWindow,
                Box::new(AppWindow::new()),
                Vec::default()
            )
            .is_ok()
        );
    
        // Initializes focus
        assert!(app.active(&Id::AppWindow).is_ok());
    
        app
    }
}

impl Default for Model {
    fn default() -> Self {
        Self { app: Self::init_app(), quit: false, redraw: true, terminal: TerminalBridge::new().expect("Cannot initialize terminal") }
    }
}

impl Update<AppMsg> for Model {
    fn update(&mut self, msg: Option<AppMsg>) -> Option<AppMsg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                AppMsg::Quit => self.quit = true,
            }
        }

        None
    }
}