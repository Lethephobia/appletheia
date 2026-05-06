use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationInvitationId,
};

use crate::authorization::{
    OrganizationInvitationRelationshipUpdater, OrganizationInvitationRelationshipUpdaterError,
};

pub struct OrganizationInvitationEventSaveHook<OIRU>
where
    OIRU: OrganizationInvitationRelationshipUpdater,
{
    organization_invitation_relationship_updater: OIRU,
}

impl<OIRU> OrganizationInvitationEventSaveHook<OIRU>
where
    OIRU: OrganizationInvitationRelationshipUpdater,
{
    pub fn new(organization_invitation_relationship_updater: OIRU) -> Self {
        Self {
            organization_invitation_relationship_updater,
        }
    }
}

impl<OIRU> EventSaveHook<OrganizationInvitation> for OrganizationInvitationEventSaveHook<OIRU>
where
    OIRU: OrganizationInvitationRelationshipUpdater,
{
    type Uow = OIRU::Uow;
    type Error = OrganizationInvitationRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<OrganizationInvitationId, OrganizationInvitationEventPayload>,
    ) -> Result<(), Self::Error> {
        if let OrganizationInvitationEventPayload::Issued {
            organization_id,
            invitee_id,
            ..
        } = event.payload()
        {
            self.organization_invitation_relationship_updater
                .upsert_organization(uow, event.aggregate_id(), *organization_id)
                .await?;
            self.organization_invitation_relationship_updater
                .upsert_invitee(uow, event.aggregate_id(), *invitee_id)
                .await?;
        }

        Ok(())
    }
}
