use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::pagination::Page;
use crate::projection::TransferRecipientListItemProjectorSpec;
use crate::read_model::{
    TransferRecipientListItem, TransferRecipientListItemCursor, TransferRecipientListItemReader,
};

use super::{TransferRecipientListQuery, TransferRecipientListQueryHandlerError};

/// Handles transfer recipient list queries.
pub struct TransferRecipientListQueryHandler<S>
where
    S: TransferRecipientListItemReader,
{
    store: S,
}

impl<S> TransferRecipientListQueryHandler<S>
where
    S: TransferRecipientListItemReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for TransferRecipientListQueryHandler<S>
where
    S: TransferRecipientListItemReader,
{
    type Query = TransferRecipientListQuery;
    type Output = Page<TransferRecipientListItem, TransferRecipientListItemCursor>;
    type Error = TransferRecipientListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[TransferRecipientListItemProjectorSpec::DESCRIPTOR]);

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
            .list(
                uow,
                query.keyword,
                query.currency_id,
                query.cursor_options,
                query.limit,
            )
            .await?)
    }
}
