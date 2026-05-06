use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{OrganizationId, OrganizationInvitationId, UserId};

use super::OrganizationInvitationRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait OrganizationInvitationRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_invitee(
        &self,
        uow: &mut Self::Uow,
        invitation_id: OrganizationInvitationId,
        invitee_id: UserId,
    ) -> Result<(), OrganizationInvitationRelationshipUpdaterError>;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        invitation_id: OrganizationInvitationId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationInvitationRelationshipUpdaterError>;
}
