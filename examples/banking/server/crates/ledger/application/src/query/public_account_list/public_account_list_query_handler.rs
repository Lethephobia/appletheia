use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::pagination::Page;
use crate::projection::PublicAccountListItemProjectorSpec;
use crate::read_model::{
    PublicAccountListItem, PublicAccountListItemCursor, PublicAccountListItemReader,
};

use super::{PublicAccountListQuery, PublicAccountListQueryHandlerError};

/// Handles public account list queries.
pub struct PublicAccountListQueryHandler<S>
where
    S: PublicAccountListItemReader,
{
    store: S,
}

impl<S> PublicAccountListQueryHandler<S>
where
    S: PublicAccountListItemReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for PublicAccountListQueryHandler<S>
where
    S: PublicAccountListItemReader,
{
    type Query = PublicAccountListQuery;
    type Output = Page<PublicAccountListItem, PublicAccountListItemCursor>;
    type Error = PublicAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[PublicAccountListItemProjectorSpec::DESCRIPTOR]);

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
            .list(uow, query.criteria, query.cursor_options, query.limit)
            .await?)
    }
}
