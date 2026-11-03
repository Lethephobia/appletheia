use crate::Retryability;
use crate::query::Query;
use crate::read_model::ReadModelDocument;

use super::{ReadModelWatchError, ReadModelWatchId, ReadModelWatchResult};

/// Owns a query watch and its persisted target registration.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatch: Send {
    type Query: Query;
    type Document: ReadModelDocument;
    type HandlerError: Retryability;

    fn watch_id(&self) -> ReadModelWatchId;

    /// Returns the initial document, then refreshed documents. Returns `Closed` once closed.
    async fn next(
        &mut self,
    ) -> Result<ReadModelWatchResult<Self::Document>, ReadModelWatchError<Self::HandlerError>>;

    /// Replaces the query immediately and requests a refresh on the next call to next.
    /// Does not execute the query or change persisted targets. Closing or closed watches reject updates.
    fn update_query(
        &mut self,
        query: Self::Query,
    ) -> Result<(), ReadModelWatchError<Self::HandlerError>>;

    /// Stops the subscription and removes its registration. Repeated calls are allowed.
    async fn close(&mut self) -> Result<(), ReadModelWatchError<Self::HandlerError>>;
}
