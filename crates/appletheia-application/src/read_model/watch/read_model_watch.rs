use std::error::Error;

use crate::read_model::ReadModelDocument;

use super::{ReadModelWatchError, ReadModelWatchId};

/// Owns the persisted watch registration and its lease lifecycle.
/// Lease renewal must remain independent of document polling.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatch: Send {
    type Document: ReadModelDocument;
    type HandlerError: Error + Send + Sync + 'static;

    fn watch_id(&self) -> ReadModelWatchId;

    /// Returns the initial document, then refreshed documents. Returns `None` once closed.
    async fn next(
        &mut self,
    ) -> Result<Option<Self::Document>, ReadModelWatchError<Self::HandlerError>>;

    /// Stops the subscription and removes its registration. Repeated calls are allowed.
    async fn close(&mut self) -> Result<(), ReadModelWatchError<Self::HandlerError>>;
}
