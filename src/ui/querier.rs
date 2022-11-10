use std::sync::mpsc::Sender;

use plugin_manager::{query::QueryInfo, PluginManager};
use youtube::YouTube;

use super::event::UserEvent;

pub enum Message {
    Data(QueryInfo),
    Quit,
}

pub struct Querier {
    tx: Sender<Message>,
    /*plugin_manager: PluginManager,
    runtime: Runtime,
    tx: Sender<UserEvent>,*/
}

impl Querier {
    pub fn new(user_event: Sender<UserEvent>) -> Result<Self, ()> {
        // Channel used for communication between external api of
        // queries and intern worker
        let (internal_tx, internal_rx) = std::sync::mpsc::channel();
        let (tmp_tx, tmp_rx) = std::sync::mpsc::channel();

        // Internal worker
        let _thread = std::thread::spawn(move || {
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
                let query = match message {
                    Message::Data(query) => query,
                    Message::Quit => break,
                };

                let result = runtime.block_on(manager.query(query));
                let _ = match result {
                    Ok(result) => user_event.send(UserEvent::QueryResult(result)),
                    Err(error) => user_event.send(UserEvent::QueryError(error.to_string())),
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
        /*let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        Ok(Self {
            plugin_manager,
            runtime,
            tx,
        })*/
    }

    pub fn query(&self, query: QueryInfo) {
        let _ = self.tx.send(Message::Data(query));
    }
}

impl Drop for Querier {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::Quit);
    }
}
