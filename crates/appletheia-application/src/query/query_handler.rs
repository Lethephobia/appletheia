use std::error::Error;

use crate::authorization::AuthorizationPlan;
use crate::projection::ProjectorDependencies;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::Query;

#[allow(async_fn_in_trait)]
pub trait QueryHandler: Send + Sync {
    const DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::None;

    type Query: Query;
    type Output: Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork;

    fn authorization_plan(&self, _query: &Self::Query) -> AuthorizationPlan {
        AuthorizationPlan::default()
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error>;
}
