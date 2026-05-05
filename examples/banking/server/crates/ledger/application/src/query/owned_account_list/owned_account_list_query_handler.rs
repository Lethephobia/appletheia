use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::AccountOwner;

use crate::pagination::Page;
use crate::projection::OwnedAccountListItemProjectorSpec;
use crate::read_model::{
    OwnedAccountListItem, OwnedAccountListItemCursor, OwnedAccountListItemReader,
};

use super::{OwnedAccountListQuery, OwnedAccountListQueryHandlerError};

/// Handles account list queries.
pub struct OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListItemReader,
{
    store: S,
}

impl<S> OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListItemReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListItemReader,
{
    type Query = OwnedAccountListQuery;
    type Output = Page<OwnedAccountListItem, OwnedAccountListItemCursor>;
    type Error = OwnedAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OwnedAccountListItemProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        match query.owner {
            AccountOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            AccountOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::System,
                    PrincipalRequirement::AuthenticatedWithRelationship(
                        RelationshipRequirement::check::<Organization>(
                            organization_id,
                            OrganizationFinanceManagerRelation::REF,
                        ),
                    ),
                ]))
            }
        }
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
                query.owner,
                query.currency_id,
                query.status,
                query.cursor_options,
                query.limit,
            )
            .await?)
    }
}
