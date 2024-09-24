use phosphorus_core::{plugin_manager::query::QueryResultData, song::Song};

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
    /// Plays a song from a playlist. This causes a reset of the playing queue
    /// and sink queue.
    PlayFromPlaylist(Song),
    /// Plays the song
    Play(Song),
    PlayPause,
    StopReproducion,
    /// Tried to use a missing song. Missing means that the song isn't
    /// in a playlist, or the queue or in the result window.
    MissingSong,
    DownloadSong(QueryResultData),
    DownloadFinished(Song),
    DownloadFailed(Song, String),
    None,
}
