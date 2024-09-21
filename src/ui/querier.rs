use std::sync::mpsc::Sender;

use phosphorus_core::{
    plugin_manager::{query::QueryInfo, PluginManager},
    song::Song,
    TrackInfo,
};

use super::UserEvent;
use phosphorus_core::plugin_manager::downloader::ProgressFollowerFn;
use youtube::YouTube;

pub enum Message {
    Search(QueryInfo),
    Download(
        String, // Song url
        String, // Song dest path
        ProgressFollowerFn,
        Sender<TrackInfo>,
        Song, // Song file for the downloaded song
    ),
    Quit,
}

/// Implements an interface to interact with `plugin_manager`. `Querier`
/// will create and internal worker that will serve queries.
///
/// In particular, queries can be sent to querier using its `query` method.
/// That method will pass the query to the internal worker, who's then
/// responsible for passing it to the `plugin_manager` and handle the result.
/// The result will be sent to the opposite receiving half of the channel
/// provided in `new`. So, the opposite half of `user_event`.
pub struct Querier {
    /// Sending half of the communication channel with the internal worker.
    tx: Sender<Message>,
}

impl Querier {
    pub fn new(user_event: Sender<UserEvent>) -> Result<Self, ()> {
        // Channel used for communication between querier and its internal
        // worker. Querier APIs will send `crate::Message`s to internal
        // worker who will receive them using `internal_rx`.
        let (internal_tx, internal_rx) = std::sync::mpsc::channel();

        // Channel used to check the correct creation and setup of the
        // internal woker. During its creation, the internal worker will
        // send `None` through the channel if some error occured. If everything
        // went as fine, one `Some(())` is sent at the end.
        let (tmp_tx, tmp_rx) = std::sync::mpsc::channel();

        // Internal worker
        let _thread = std::thread::spawn(move || {
            // Creates the plugin manager and sets plugins
            let mut manager = PluginManager::new();
            if let Err(msg) = manager.register_plugin(Box::new(YouTube {}), "YouTube") {
                eprintln!(
                    "An error occured while trying to register a plugin:\n{}",
                    msg
                );
                let _ = tmp_tx.send(None);
                std::process::exit(1);
            }
            if let Err(msg) = manager.set_default("YouTube") {
                eprintln!(
                    "An error occured while trying to set the default plugin\n{}",
                    msg
                );
                let _ = tmp_tx.send(None);
                std::process::exit(1);
            }

            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();

            let runtime = match runtime {
                Ok(runtime) => runtime,
                Err(_) => {
                    eprintln!("Error in runtime creation");
                    let _ = tmp_tx.send(None);
                    std::process::exit(1);
                }
            };

            // Everythig was fine and the plugin manager has been created
            let _ = tmp_tx.send(Some(()));

            loop {
                let message = internal_rx.recv().unwrap_or(Message::Quit);
                match message {
                    Message::Search(query) => {
                        let result = runtime.block_on(manager.query(query));
                        let _ = match result {
                            Ok(result) => user_event.send(UserEvent::QueryResult(result)),
                            Err(error) => user_event.send(UserEvent::QueryError(error.to_string())),
                        };
                    }
                    Message::Download(
                        url,
                        file_name,
                        progress_follower,
                        progress_forwarder,
                        song,
                    ) => {
                        let _ = runtime.block_on(manager.download(
                            &url,
                            &file_name,
                            progress_follower,
                            progress_forwarder,
                            song,
                        ));
                    }
                    Message::Quit => break,
                };
            }
        });

        // If `Some(())` is received, the plugin manager has been created successfully
        // and the worker will be able to process queries. Otherwise, the worker is
        // shutdown and the caller must be informed of the error.
        if let Ok(Some(())) = tmp_rx.recv() {
            return Ok(Self { tx: internal_tx });
        }

        Err(())
    }

    /// Sends a query to the internal worker who will serve it.
    pub fn query(&self, query: QueryInfo) {
        let _ = self.tx.send(Message::Search(query));
    }

    pub fn download(
        &self,
        url: String,
        file_name: String,
        progress_follower: ProgressFollowerFn,
        progress_forwarder: Sender<TrackInfo>,
        song: Song,
    ) {
        let _ = self.tx.send(Message::Download(
            url,
            file_name,
            progress_follower,
            progress_forwarder,
            song,
        ));
    }
}

impl Drop for Querier {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::Quit);
    }
}
