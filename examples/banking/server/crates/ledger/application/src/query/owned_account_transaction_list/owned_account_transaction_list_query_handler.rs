use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_application::{OrganizationFragmentProjectorSpec, UserFragmentProjectorSpec};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::AccountOwner;

use crate::projection::{
    AccountFragmentProjectorSpec, AccountTransactionFragmentProjectorSpec,
    CurrencyFragmentProjectorSpec,
};
use crate::read_model::OwnedAccountTransactionList;
use crate::read_model::OwnedAccountTransactionListReader;

use super::{OwnedAccountTransactionListQuery, OwnedAccountTransactionListQueryHandlerError};

/// Handles owned account transaction list queries.
pub struct OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListReader,
{
    reader: S,
}

impl<S> OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for OwnedAccountTransactionListQueryHandler<S>
where
    S: OwnedAccountTransactionListReader,
{
    type Query = OwnedAccountTransactionListQuery;
    type Output = OwnedAccountTransactionList;
    type Error = OwnedAccountTransactionListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        AccountFragmentProjectorSpec::DESCRIPTOR,
        CurrencyFragmentProjectorSpec::DESCRIPTOR,
        UserFragmentProjectorSpec::DESCRIPTOR,
        OrganizationFragmentProjectorSpec::DESCRIPTOR,
        AccountTransactionFragmentProjectorSpec::DESCRIPTOR,
    ]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        match query.owner {
            AccountOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            AccountOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
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
            .list(uow, query.owner, query.criteria, query.sort, query.page)
            .await?)
    }
}
