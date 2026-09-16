use super::{
    ReadModelWatch, ReadModelWatchEndpoint, ReadModelWatchId, ReadModelWatchRegistryError,
    ReadModelWatchSession, ReadModelWatchSessionId,
};

/// Manages local sessions and synchronizes their desired routing registrations.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchRegistry: Send + Sync {
    fn endpoint(&self) -> ReadModelWatchEndpoint;

    fn open_session(&mut self) -> ReadModelWatchSessionId;

    fn session(&self, id: ReadModelWatchSessionId) -> Option<&ReadModelWatchSession>;

    /// Adds the watch locally only after index registration succeeds.
    /// On failure, the caller may retry with a retained clone of the watch.
    async fn register_watch(
        &mut self,
        session_id: ReadModelWatchSessionId,
        watch: ReadModelWatch,
    ) -> Result<(), ReadModelWatchRegistryError>;

    async fn unregister_watch(
        &mut self,
        session_id: ReadModelWatchSessionId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError>;

    async fn close_session(
        &mut self,
        session_id: ReadModelWatchSessionId,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Retries desired registrations and renews the endpoint lease.
    async fn synchronize_index(&mut self) -> Result<(), ReadModelWatchRegistryError>;
}
