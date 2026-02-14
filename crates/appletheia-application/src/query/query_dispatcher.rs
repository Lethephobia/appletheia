use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::{QueryConsistency, QueryDispatchError, QueryHandler};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct QueryOptions {
    pub consistency: QueryConsistency,
}

#[allow(async_fn_in_trait)]
pub trait QueryDispatcher: Send + Sync {
    type Uow: UnitOfWork;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatchError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>;
}
