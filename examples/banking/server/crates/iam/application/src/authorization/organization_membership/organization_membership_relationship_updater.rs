use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationMembershipId, OrganizationRoles, UserId};

use super::OrganizationMembershipRelationshipUpdaterError;

/// Derives organization member and role relationships from membership state.
#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    /// Links the membership aggregate to its organization so that
    /// membership-scoped relations can resolve organization authority.
    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        organization_membership_id: OrganizationMembershipId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn upsert_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn replace_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &OrganizationRoles,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_all_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;
}
