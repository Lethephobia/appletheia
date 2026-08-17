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
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;

use crate::projection::WalletBookmarkFragmentProjectorSpec;
use crate::read_model::{WalletBookmarkList, WalletBookmarkListReader};

use super::{WalletBookmarkListQuery, WalletBookmarkListQueryHandlerError};

/// Handles wallet bookmark list queries.
pub struct WalletBookmarkListQueryHandler<S>
where
    S: WalletBookmarkListReader,
{
    reader: S,
}

impl<S> WalletBookmarkListQueryHandler<S>
where
    S: WalletBookmarkListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for WalletBookmarkListQueryHandler<S>
where
    S: WalletBookmarkListReader,
{
    type Query = WalletBookmarkListQuery;
    type Output = WalletBookmarkList;
    type Error = WalletBookmarkListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[WalletBookmarkFragmentProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        match query.owner {
            WalletBookmarkOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            WalletBookmarkOwner::Organization(organization_id) => {
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
