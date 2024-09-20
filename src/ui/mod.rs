use phosphorus_core::{
    playlist_manager::PlaylistManager,
    queue::QueueManager,
    song::{Song, SongDetails},
};
use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use phosphorus_core::plugin_manager::query::{QueryInfo, QueryResult, QueryResultData};
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, EventListenerCfg, Sub, SubEventClause, Update,
};

use crate::{
    config::Paths,
    player::Player,
    ui::{
        app_window::AppWindow, event::UserEventPort, player_bar::PlayerBar, status_bar::StatusBar,
        top_bar::TopBar,
    },
};

use self::{event::UserEvent, querier::Querier};

mod app_window;
mod event;
mod player_bar;
mod playlist_list;
mod querier;
mod queue;
mod secondary_window;
mod status_bar;
mod top_bar;
mod welcome_window;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Label,
    AppWindow,
    TopBar,
    StatusBar,
    PlayerBar,
}

#[derive(Debug, PartialEq)]
pub enum AppMsg {
    /// Closes the application
    Quit,
    /// The current active componen loses its focus
    LoseFocus,
    /// Can be used when a secondary window is closed
    ResetFocus,
    /// The focus is passed to the next component
    GoNextItem,
    /// Is like calling GoNext n time, so GoNextItem
    /// is equivalent to GoForward(1)
    GoForward(u16),
    /// The help window has been requested
    ShowHelp,
    /// Show songs in a playlist
    ShowPlaylist,
    /// Boh
    QuerySent(String),
    /// Plays&downloads a song retrieving it from query results
    PlayFromResult(QueryResultData),
    /// Plays a song from a playlist
    PlayFromPlaylist(usize),
    /// Plays the song
    Play(Song),
    PlayPause,
    /// Tried to use a missing song. Missing means that the song isn't
    /// in a playlist, or the queue or in the result window.
    MissingSong,
    DownloadSong(QueryResultData),
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
            FocusableItem::SearchBar => Id::TopBar,
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
    pub app: Application<Id, AppMsg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
    // Used to track the active component
    active: FocusableItem,
    is_secondary_window_active: bool,
    paths: Paths,
    user_event: Sender<UserEvent>,
    /// Used to send queries to plugin manager
    querier: Querier,
    /// Used to reproduce audio files
    player: Player,
    /// Used to track reproduction state:
    /// None: no song is being played
    /// Some(true): a song is current being played
    /// Some(false): a song is being played, but has been paused
    playing: Option<bool>,
}

impl Model {
    pub fn new(
        paths: Paths,
        playlist_manager: PlaylistManager,
        queue_manager: QueueManager,
    ) -> Result<Self, ()> {
        let (tx, rx) = std::sync::mpsc::channel();
        let querier = Querier::new(tx.clone())?;

        Ok(Self {
            app: Self::init_app(playlist_manager, queue_manager, rx),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            active: FocusableItem::SearchBar,
            is_secondary_window_active: false,
            paths,
            user_event: tx,
            querier,
            player: Player::try_new().expect("Cannot initialize the player process"),
            playing: None,
        })
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
                            Constraint::Length(3), // PlayerBar
                            Constraint::Length(1), // StatusBar
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::TopBar, f, chunks[0]);
                self.app.view(&Id::AppWindow, f, chunks[1]);
                self.app.view(&Id::PlayerBar, f, chunks[2]);
                self.app.view(&Id::StatusBar, f, chunks[3]);
            })
            .is_ok());
    }

    pub fn init_app(
        playlist_manager: PlaylistManager,
        queue_manager: QueueManager,
        rx: Receiver<UserEvent>,
    ) -> Application<Id, AppMsg, UserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock
        let mut app: Application<Id, AppMsg, UserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .port(UserEventPort::new(rx).boxed(), Duration::from_millis(100))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(50)),
        );

        // Mounts the components
        assert!(app
            .mount(Id::TopBar, TopBar::default().boxed(), Vec::default())
            .is_ok());
        assert!(app
            .mount(
                Id::AppWindow,
                Box::new(AppWindow::new(playlist_manager, queue_manager)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(Id::PlayerBar, PlayerBar::default().boxed(), Vec::default())
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
                &Id::AppWindow,
                Sub::new(
                    SubEventClause::User(UserEvent::SecondaryWindowClosed),
                    tuirealm::SubClause::Always
                )
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::AppWindow,
                Sub::new(
                    SubEventClause::User(UserEvent::QueryResult(QueryResult::default())),
                    tuirealm::SubClause::Always
                )
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::PlayerBar,
                Sub::new(
                    SubEventClause::User(UserEvent::PlaySong(Song::default())),
                    tuirealm::SubClause::Always
                )
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::PlayerBar,
                Sub::new(SubEventClause::Tick, tuirealm::SubClause::Always)
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::StatusBar,
                Sub::new(SubEventClause::Any, tuirealm::SubClause::Always)
            )
            .is_ok());

        assert!(app
            .subscribe(
                &Id::TopBar,
                Sub::new(
                    SubEventClause::User(UserEvent::DownloadRegistered(String::default())),
                    tuirealm::SubClause::Always
                )
            )
            .is_ok());

        // Initializes focus on search bar
        assert!(app.active(&Id::TopBar).is_ok());

        app
    }

    /// A utility function which uses `QueryResultData` to create a `Song`
    /// instance associated to the file `file_name.mp3` in the `Paths.download`
    /// folder. The relative meta-file has the same name as the 'raw' one, but
    /// is a json file withing the `Paths.data` directory.
    fn create_song_file(&self, query_data: &QueryResultData, file_name: &str) -> Song {
        let mp3 = self.paths.download().join(
            format!("{}.mp3", file_name)
                .to_lowercase()
                .replace(" ", "_"),
        );
        let json = self.paths.data().join(
            format!("{}.json", file_name)
                .to_lowercase()
                .replace(" ", "_"),
        );

        let song = Song::new(
            mp3.to_str().unwrap(),
            json.to_str().unwrap(),
            SongDetails::new(
                query_data.track_name(),
                Some(query_data.artist_name()),
                None,
                Some(query_data.duration().clone()),
            ),
        );
        song
    }
}

impl Update<AppMsg> for Model {
    fn update(&mut self, msg: Option<AppMsg>) -> Option<AppMsg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                AppMsg::Quit => self.quit = true,
                AppMsg::LoseFocus => {
                    if self.is_secondary_window_active {
                        self.is_secondary_window_active = false;
                        let _ = self.user_event.send(UserEvent::SecondaryWindowClosed);
                        if let Some(item) = self.active.below_item() {
                            self.active = item;
                            assert!(self.app.active(&self.active.to_id()).is_ok());
                        }
                    }
                }
                AppMsg::ResetFocus => {
                    self.active = FocusableItem::SearchBar;
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::GoNextItem => {
                    self.active = self.active.next();
                    // This if statement is necessary to display components
                    //  into the container as not focused when another component
                    //  of the same container is focused.
                    //  DO NOT REMOVE
                    if let FocusableItem::MainWindow = self.active {
                        if self.is_secondary_window_active {
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
                    let _ = self.user_event.send(UserEvent::HelpOpened);
                    self.active = FocusableItem::SecondaryWindow;
                    self.is_secondary_window_active = true;

                    // Again, I don't know why this has to repeted
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::ShowPlaylist => {
                    let _ = self.user_event.send(UserEvent::PlaylistViewOpened);
                    self.active = FocusableItem::SecondaryWindow;
                    self.is_secondary_window_active = true;

                    // Again, I don't know why this has to repeted
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                    assert!(self.app.active(&self.active.to_id()).is_ok());
                }
                AppMsg::QuerySent(query) => {
                    let query = QueryInfo::as_raw(&query);
                    self.querier.query(query);
                    let _ = self.user_event.send(UserEvent::QuerySent);
                }
                AppMsg::Play(song) => {
                    self.player.initiate(&song).expect("Error in reproduction!");
                    let _ = self.user_event.send(UserEvent::PlaySong(song));
                    self.playing = Some(true);
                }
                AppMsg::DownloadSong(query_data) => {
                    // Let download tracker know about the new download
                    let _ = self.user_event.send(UserEvent::DownloadRegistered(
                        query_data.track_name().to_string(),
                    ));
                    let file_name = phosphorus_core::file_name_from_basics(
                        query_data.track_name(),
                        query_data.artist_name(),
                    );
                    let song = self.create_song_file(&query_data, &file_name);

                    let raw_path = self.paths.download().join(&file_name);
                    self.querier.download(
                        query_data.track_url().to_string(),
                        raw_path.to_str().unwrap().to_string(),
                        |rx| {
                            loop {
                                match rx.recv() {
                                    Ok(value) => {
                                        if value == 100.0 {
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        // The sender has terminated sending data
                                        break;
                                    }
                                }
                            }
                        },
                    );
                    let _ = self.user_event.send(UserEvent::DownloadFinished(song));
                }
                _ => (),
            }
        }

        None
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        //let _ = self.terminal.disable_raw_mode();
        //let _ = self.terminal.leave_alternate_screen();
        //self.quit = true;
    }
}
