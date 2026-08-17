use std::collections::HashSet;

use crate::read_model::SerializedPartition;

use super::{ReadModelWatchSession, ReadModelWatchSessionId};

/// Stores process-local watch sessions and their active delivery endpoints.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchSessionRegistry: Clone + Send + Sync {
    async fn register(&self, session_id: ReadModelWatchSessionId, session: ReadModelWatchSession);

    async fn remove(&self, session_id: &ReadModelWatchSessionId);

    async fn session(&self, session_id: &ReadModelWatchSessionId) -> Option<ReadModelWatchSession>;

    async fn session_ids_for_partition(
        &self,
        partition: &SerializedPartition,
    ) -> Vec<ReadModelWatchSessionId>;

    async fn replace_partition_index(
        &self,
        session_id: ReadModelWatchSessionId,
        old_partitions: HashSet<SerializedPartition>,
        new_partitions: HashSet<SerializedPartition>,
    );
}
