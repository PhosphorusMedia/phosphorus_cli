use std::sync::mpsc::Sender;

use plugin_manager::{query::QueryInfo, PluginManager};
use tokio::runtime::Runtime;

use super::event::UserEvent;

pub struct Querier {
    plugin_manager: PluginManager,
    runtime: Runtime,
    tx: Sender<UserEvent>,
}

impl Querier {
    pub fn new(
        plugin_manager: PluginManager,
        tx: Sender<UserEvent>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        Ok(Self {
            plugin_manager,
            runtime,
            tx,
        })
    }

    pub fn query(&self, query: QueryInfo) {
        let result = self.runtime.block_on(self.plugin_manager.query(query));
        let _ = match result {
            Ok(result) => self.tx.send(UserEvent::QueryResult(result)),
            Err(error) => self.tx.send(UserEvent::QueryError(error.to_string())),
        };
    }
}
