use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::{QueryDispatcherError, QueryHandler};

#[allow(async_fn_in_trait)]
pub trait QueryDispatcher: Send + Sync {
    type Uow: UnitOfWork;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
    ) -> Result<H::Output, QueryDispatcherError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>;
}
