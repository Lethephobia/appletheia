use std::error::Error;

use crate::read_model::ReadModelDocument;

use super::{ReadModelWatchError, ReadModelWatchId, ReadModelWatchResult};

/// Owns a query watch and its persisted target registration.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatch: Send {
    type Document: ReadModelDocument;
    type HandlerError: Error + Send + Sync + 'static;

    fn watch_id(&self) -> ReadModelWatchId;

    /// Returns the initial document, then refreshed documents. Returns `Closed` once closed.
    async fn next(
        &mut self,
    ) -> Result<ReadModelWatchResult<Self::Document>, ReadModelWatchError<Self::HandlerError>>;

    /// Stops the subscription and removes its registration. Repeated calls are allowed.
    async fn close(&mut self) -> Result<(), ReadModelWatchError<Self::HandlerError>>;
}
