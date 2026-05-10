use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::PublicAccountListProjectorSpec;
use crate::read_model::{PublicAccountList, PublicAccountListReader};

use super::{PublicAccountListQuery, PublicAccountListQueryHandlerError};

/// Handles public account list queries.
pub struct PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    store: S,
}

impl<S> PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    type Query = PublicAccountListQuery;
    type Output = PublicAccountList;
    type Error = PublicAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[PublicAccountListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::None)
    }

    async fn handle(
        &self,
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .store
            .list(uow, query.criteria, query.cursor_options, query.limit)
            .await?)
    }
}
