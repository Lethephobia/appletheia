use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationMembershipId, OrganizationRole, UserId};

use super::OrganizationMembershipRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait OrganizationMembershipRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        membership_id: OrganizationMembershipId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_organization(
        &self,
        uow: &mut Self::Uow,
        membership_id: OrganizationMembershipId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn upsert_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &[OrganizationRole],
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn upsert_role(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_role(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;

    async fn remove_all_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError>;
}
