use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{
    OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipId,
};

use crate::authorization::{
    OrganizationMembershipRelationshipUpdater, OrganizationMembershipRelationshipUpdaterError,
};

pub struct OrganizationMembershipEventSaveHook<OMRU>
where
    OMRU: OrganizationMembershipRelationshipUpdater,
{
    organization_membership_relationship_updater: OMRU,
}

impl<OMRU> OrganizationMembershipEventSaveHook<OMRU>
where
    OMRU: OrganizationMembershipRelationshipUpdater,
{
    pub fn new(organization_membership_relationship_updater: OMRU) -> Self {
        Self {
            organization_membership_relationship_updater,
        }
    }
}

impl<OMRU> EventSaveHook<OrganizationMembership> for OrganizationMembershipEventSaveHook<OMRU>
where
    OMRU: OrganizationMembershipRelationshipUpdater,
{
    type Uow = OMRU::Uow;
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
                ..
            } => {
                updater
                    .upsert_organization(uow, event.aggregate_id(), *organization_id)
                    .await?;
                updater
                    .upsert_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .replace_roles(uow, *organization_id, *user_id, roles)
                    .await?;
            }
            OrganizationMembershipEventPayload::Activated {
                organization_id,
                user_id,
                roles,
            } => {
                updater
                    .upsert_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .replace_roles(uow, *organization_id, *user_id, roles)
                    .await?;
            }
            OrganizationMembershipEventPayload::Inactivated {
                organization_id,
                user_id,
            } => {
                updater
                    .remove_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .remove_all_roles(uow, *organization_id, *user_id)
                    .await?;
            }
            OrganizationMembershipEventPayload::RolesChanged {
                organization_id,
                user_id,
                roles,
            } => {
                updater
                    .replace_roles(uow, *organization_id, *user_id, roles)
                    .await?;
            }
            OrganizationMembershipEventPayload::Removed {
                organization_id,
                user_id,
            } => {
                updater
                    .remove_organization(uow, event.aggregate_id())
                    .await?;
                updater
                    .remove_member(uow, *organization_id, *user_id)
                    .await?;
                updater
                    .remove_all_roles(uow, *organization_id, *user_id)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
