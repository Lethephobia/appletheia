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
        if let UserEventPayload::Registered { id, .. } = event.payload() {
            self.user_relationship_updater
                .upsert_owner(uow, *id)
                .await?;
        }

        Ok(())
    }
}
