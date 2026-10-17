use crate::query::QueryHandler;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::{ReadModelWatch, ReadModelWatcherError};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatcher: Send + Sync {
    type Uow: UnitOfWork;
    type Watch<H>: ReadModelWatch<Document = H::Output, HandlerError = H::Error>
    where
        H: QueryHandler<Uow = Self::Uow>;

    /// Starts a query subscription whose first result and subsequent refreshed documents
    /// are received through `next`.
    async fn watch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
    ) -> Result<Self::Watch<H>, ReadModelWatcherError>
    where
        H: QueryHandler<Uow = Self::Uow>;
}
