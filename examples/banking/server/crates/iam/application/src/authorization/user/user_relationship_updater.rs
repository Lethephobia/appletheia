use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};

use super::UserRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait UserRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError>;

    async fn upsert_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError>;

    async fn remove_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError>;

    async fn replace_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &OrganizationRoles,
    ) -> Result<(), UserRelationshipUpdaterError>;

    async fn remove_all_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError>;
}
