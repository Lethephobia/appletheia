use crate::read_model::ReadModelFragmentChangeEnvelope;

use super::{
    ReadModelFragmentChangeDispatcher, ReadModelWatchFragmentDispatcherError,
    ReadModelWatchSessionRegistry,
};

/// Fans one source-partition change out through the registered typed sessions.
pub struct DefaultReadModelFragmentChangeDispatcher<R> {
    registry: R,
}

impl<R> DefaultReadModelFragmentChangeDispatcher<R> {
    pub fn new(registry: R) -> Self {
        Self { registry }
    }
}

impl<R> ReadModelFragmentChangeDispatcher for DefaultReadModelFragmentChangeDispatcher<R>
where
    R: ReadModelWatchSessionRegistry,
{
    async fn dispatch(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
    ) -> Result<(), ReadModelWatchFragmentDispatcherError> {
        let session_ids = self
            .registry
            .session_ids_for_partition(&envelope.partition)
            .await;
        for session_id in session_ids {
            let Some(session) = self.registry.session(&session_id).await else {
                continue;
            };
            let (_route, old_partitions, new_partitions) = session
                .dispatch(envelope)
                .await
                .map_err(|source| ReadModelWatchFragmentDispatcherError { session_id, source })?;
            self.registry
                .replace_partition_index(session_id, old_partitions, new_partitions)
                .await;
        }
        Ok(())
    }
}
