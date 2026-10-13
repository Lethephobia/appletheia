use crate::read_model::ReadModelRef;

use super::{ReadModelWatch, ReadModelWatchId, ReadModelWatchRegistryError};

/// Stores local watches and indexes them by target. Callers must serialize updates
/// and removal of the same watch; saving a watch replaces its stored state.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchRegistry: Send + Sync {
    async fn save(&self, watch: ReadModelWatch) -> Result<(), ReadModelWatchRegistryError>;

    async fn find_by_id(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<Option<ReadModelWatch>, ReadModelWatchRegistryError>;

    async fn find_by_target(
        &self,
        target: &ReadModelRef,
    ) -> Result<Vec<ReadModelWatchId>, ReadModelWatchRegistryError>;

    async fn remove(&self, watch_id: ReadModelWatchId) -> Result<(), ReadModelWatchRegistryError>;
}
