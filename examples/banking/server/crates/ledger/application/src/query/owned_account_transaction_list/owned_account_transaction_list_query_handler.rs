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
use crate::projection::{
    OwnedAccountListItemProjectorSpec, OwnedAccountTransactionListItemProjectorSpec,
};
use crate::read_model::{
    OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCursor,
    OwnedAccountTransactionListItemReader,
};

use super::{OwnedAccountTransactionListQuery, OwnedAccountTransactionListQueryHandlerError};

/// Handles owned account transaction list queries.
pub struct OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListItemReader,
{
    reader: S,
}

impl<S> OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListItemReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListItemReader,
{
    type Query = OwnedAccountTransactionListQuery;
    type Output = Page<OwnedAccountTransactionListItem, OwnedAccountTransactionListItemCursor>;
    type Error = OwnedAccountTransactionListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        OwnedAccountListItemProjectorSpec::DESCRIPTOR,
        OwnedAccountTransactionListItemProjectorSpec::DESCRIPTOR,
    ]);

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
            .reader
            .list(
                uow,
                query.owner,
                query.account_id,
                query.currency_id,
                query.status,
                query.cursor_options,
                query.limit,
            )
            .await?)
    }
}
