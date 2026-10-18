use crate::query::QueryHandler;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::{ReadModelWatch, ReadModelWatcherError};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatcher: Send + Sync {
    type Uow: UnitOfWork;
    type Watch<'a, H>: ReadModelWatch<Document = H::Output, HandlerError = H::Error>
    where
        Self: 'a,
        H: QueryHandler<Uow = Self::Uow> + 'a;

    /// Starts a query subscription whose first result and subsequent refreshed documents
    /// are received through `next`.
    async fn watch<'a, H>(
        &'a self,
        handler: &'a H,
        request_context: &'a RequestContext,
        query: H::Query,
    ) -> Result<Self::Watch<'a, H>, ReadModelWatcherError>
    where
        H: QueryHandler<Uow = Self::Uow> + 'a;
}
