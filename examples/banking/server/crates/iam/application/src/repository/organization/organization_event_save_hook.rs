use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{Organization, OrganizationEventPayload, OrganizationId};

use crate::authorization::{OrganizationRelationshipUpdater, OrganizationRelationshipUpdaterError};

pub struct OrganizationEventSaveHook<ORU>
where
    ORU: OrganizationRelationshipUpdater,
{
    organization_relationship_updater: ORU,
}

impl<ORU> OrganizationEventSaveHook<ORU>
where
    ORU: OrganizationRelationshipUpdater,
{
    pub fn new(organization_relationship_updater: ORU) -> Self {
        Self {
            organization_relationship_updater,
        }
    }
}

impl<ORU> EventSaveHook<Organization> for OrganizationEventSaveHook<ORU>
where
    ORU: OrganizationRelationshipUpdater,
{
    type Uow = ORU::Uow;
    type Error = OrganizationRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<OrganizationId, OrganizationEventPayload>,
    ) -> Result<(), Self::Error> {
        match event.payload() {
            OrganizationEventPayload::Created { owner, .. } => {
                self.organization_relationship_updater
                    .upsert_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                self.organization_relationship_updater
                    .replace_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
