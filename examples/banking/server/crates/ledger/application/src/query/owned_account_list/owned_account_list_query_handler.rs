use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_application::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
    UserOwnerRelationshipProjectorSpec,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::AccountOwner;

use crate::projection::{AccountProjectorSpec, CurrencyProjectorSpec};
use crate::query::Page;

use super::{
    OwnedAccountListCursor, OwnedAccountListItem, OwnedAccountListQuery,
    OwnedAccountListQueryHandlerError, OwnedAccountListStore,
};

/// Handles account list queries.
pub struct OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListStore,
{
    store: S,
}

impl<S> OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListStore,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for OwnedAccountListQueryHandler<S>
where
    S: OwnedAccountListStore,
{
    type Query = OwnedAccountListQuery;
    type Output = Page<OwnedAccountListItem, OwnedAccountListCursor>;
    type Error = OwnedAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        <AccountProjectorSpec as ProjectorSpec>::DESCRIPTOR,
        <CurrencyProjectorSpec as ProjectorSpec>::DESCRIPTOR,
    ]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        match query.owner {
            AccountOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<User>(
                        user_id,
                        UserOwnerRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])),
            AccountOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::System,
                    PrincipalRequirement::AuthenticatedWithRelationship {
                        requirement: RelationshipRequirement::check::<Organization>(
                            organization_id,
                            OrganizationFinanceManagerRelation::REF,
                        ),
                        projector_dependencies: ProjectorDependencies::Some(&[
                            OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                            OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                        ]),
                    },
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
        self.store.list(uow, &query).await.map_err(Into::into)
    }
}
