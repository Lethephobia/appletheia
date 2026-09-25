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

use crate::projection::{AccountFragmentProjectorSpec, CurrencyFragmentProjectorSpec};
use crate::read_model::{OwnedAccountList, OwnedAccountListReader};

use super::{OwnedAccountListQuery, OwnedAccountListQueryHandlerError};

/// Handles account list queries.
pub struct OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListReader,
{
    reader: S,
}

impl<S> OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListReader,
{
    type Query = OwnedAccountListQuery;
    type Output = OwnedAccountList;
    type Error = OwnedAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        AccountFragmentProjectorSpec::DESCRIPTOR,
        CurrencyFragmentProjectorSpec::DESCRIPTOR,
        UserFragmentProjectorSpec::DESCRIPTOR,
        OrganizationFragmentProjectorSpec::DESCRIPTOR,
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
