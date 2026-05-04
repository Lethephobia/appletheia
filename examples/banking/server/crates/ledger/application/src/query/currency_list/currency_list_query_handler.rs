use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::pagination::Page;
use crate::projection::CurrencyListItemProjectorSpec;
use crate::read_model::{CurrencyListItem, CurrencyListItemCursor, CurrencyListItemReader};

use super::{CurrencyListQuery, CurrencyListQueryHandlerError};

/// Handles public currency list queries.
pub struct CurrencyListQueryHandler<S>
where
    S: CurrencyListItemReader,
{
    store: S,
}

impl<S> CurrencyListQueryHandler<S>
where
    S: CurrencyListItemReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for CurrencyListQueryHandler<S>
where
    S: CurrencyListItemReader,
{
    type Query = CurrencyListQuery;
    type Output = Page<CurrencyListItem, CurrencyListItemCursor>;
    type Error = CurrencyListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[CurrencyListItemProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .store
            .list(uow, query.status, query.cursor_options, query.limit)
            .await?)
    }
}
