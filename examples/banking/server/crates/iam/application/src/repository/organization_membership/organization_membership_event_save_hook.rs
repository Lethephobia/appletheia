use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{
    OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipId,
};

use crate::authorization::{
    OrganizationMembershipRelationshipUpdater, OrganizationMembershipRelationshipUpdaterError,
};

/// Derives organization member and role relationships from membership events.
///
/// Deriving them per membership aggregate keeps one member's update from
/// touching relationships owned by another membership.
pub struct OrganizationMembershipEventSaveHook<MRU>
where
    MRU: OrganizationMembershipRelationshipUpdater,
{
    organization_membership_relationship_updater: MRU,
}

impl<MRU> OrganizationMembershipEventSaveHook<MRU>
where
    MRU: OrganizationMembershipRelationshipUpdater,
{
    pub fn new(organization_membership_relationship_updater: MRU) -> Self {
        Self {
            organization_membership_relationship_updater,
        }
    }
}

impl<MRU> EventSaveHook<OrganizationMembership> for OrganizationMembershipEventSaveHook<MRU>
where
    MRU: OrganizationMembershipRelationshipUpdater,
{
    type Uow = MRU::Uow;
    type Error = OrganizationMembershipRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<OrganizationMembershipId, OrganizationMembershipEventPayload>,
    ) -> Result<(), Self::Error> {
        let updater = &self.organization_membership_relationship_updater;

        match event.payload() {
            OrganizationMembershipEventPayload::Created {
                organization_id,
                user_id,
                roles,
            } => {
                updater
                    .upsert_organization(uow, event.aggregate_id(), *organization_id)
                    .await?;
                updater
                    .upsert_organization_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .replace_organization_roles(uow, *organization_id, *user_id, roles)
                    .await?;
            }
            OrganizationMembershipEventPayload::RolesChanged {
                organization_id,
                user_id,
                roles,
            } => {
                updater
                    .replace_organization_roles(uow, *organization_id, *user_id, roles)
                    .await?;
            }
            OrganizationMembershipEventPayload::Removed {
                organization_id,
                user_id,
            } => {
                updater
                    .remove_organization_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .remove_all_organization_roles(uow, *organization_id, *user_id)
                    .await?;
            }
        }

        Ok(())
    }
}
