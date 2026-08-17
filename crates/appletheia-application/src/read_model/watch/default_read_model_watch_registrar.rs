use std::sync::Arc;

use tokio::sync::RwLock;

use super::read_model_watch_session_registry_state::ReadModelWatchSessionRegistryState;
use super::{
    ReadModelWatchRegistrar, ReadModelWatchRegistrationError, ReadModelWatchSelection,
    ReadModelWatchSessionId,
};

/// Registers materialized read-model selections in the active session registry.
#[derive(Clone)]
pub struct DefaultReadModelWatchRegistrar {
    state: Arc<RwLock<ReadModelWatchSessionRegistryState>>,
}

impl DefaultReadModelWatchRegistrar {
    #[cfg(test)]
    pub(super) fn new(state: Arc<RwLock<ReadModelWatchSessionRegistryState>>) -> Self {
        Self { state }
    }
}

impl ReadModelWatchRegistrar for DefaultReadModelWatchRegistrar {
    async fn register(
        &self,
        session_id: &ReadModelWatchSessionId,
        selection: ReadModelWatchSelection,
    ) -> Result<(), ReadModelWatchRegistrationError> {
        let session = self
            .state
            .read()
            .await
            .sessions
            .get(session_id)
            .cloned()
            .ok_or(ReadModelWatchRegistrationError::SessionNotFound(
                *session_id,
            ))?;
        let (old_partitions, new_partitions) = session.replace_selection(selection).await?;
        self.state.write().await.replace_partition_index(
            *session_id,
            old_partitions,
            new_partitions,
        );
        Ok(())
    }
}
