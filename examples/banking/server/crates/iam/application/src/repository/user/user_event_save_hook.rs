use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{User, UserEventPayload, UserId};

use crate::authorization::{UserRelationshipUpdater, UserRelationshipUpdaterError};

pub struct UserEventSaveHook<URU>
where
    URU: UserRelationshipUpdater,
{
    user_relationship_updater: URU,
}

impl<URU> UserEventSaveHook<URU>
where
    URU: UserRelationshipUpdater,
{
    pub fn new(user_relationship_updater: URU) -> Self {
        Self {
            user_relationship_updater,
        }
    }
}

impl<URU> EventSaveHook<User> for UserEventSaveHook<URU>
where
    URU: UserRelationshipUpdater,
{
    type Uow = URU::Uow;
    type Error = UserRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<UserId, UserEventPayload>,
    ) -> Result<(), Self::Error> {
        let updater = &self.user_relationship_updater;

        match event.payload() {
            UserEventPayload::Registered { .. } => {
                updater.upsert_owner(uow, event.aggregate_id()).await?;
            }
            UserEventPayload::OrganizationMembershipGranted {
                organization_id,
                roles,
            } => {
                updater
                    .upsert_organization_member(uow, *organization_id, event.aggregate_id())
                    .await?;
                updater
                    .replace_organization_roles(uow, *organization_id, event.aggregate_id(), roles)
                    .await?;
            }
            UserEventPayload::OrganizationMembershipRolesChanged {
                organization_id,
                roles,
            } => {
                updater
                    .replace_organization_roles(uow, *organization_id, event.aggregate_id(), roles)
                    .await?;
            }
            UserEventPayload::OrganizationMembershipRemoved { organization_id } => {
                updater
                    .remove_organization_member(uow, *organization_id, event.aggregate_id())
                    .await?;
                updater
                    .remove_all_organization_roles(uow, *organization_id, event.aggregate_id())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
