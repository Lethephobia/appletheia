use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationOwner};

use super::OrganizationRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait OrganizationRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationRelationshipUpdaterError>;

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        owner: OrganizationOwner,
    ) -> Result<(), OrganizationRelationshipUpdaterError>;
}
