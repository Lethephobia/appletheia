use std::error::Error;

use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::{Query, ReadModel};

#[allow(async_fn_in_trait)]
pub trait QueryHandler: Send + Sync {
    type Query: Query;
    type ReadModel: ReadModel;
    type Output: Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork;

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error>;
}
