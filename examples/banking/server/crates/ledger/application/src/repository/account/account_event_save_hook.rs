use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountId};

use crate::authorization::{AccountRelationshipUpdater, AccountRelationshipUpdaterError};

pub struct AccountEventSaveHook<ARU>
where
    ARU: AccountRelationshipUpdater,
{
    account_relationship_updater: ARU,
}

impl<ARU> AccountEventSaveHook<ARU>
where
    ARU: AccountRelationshipUpdater,
{
    pub fn new(account_relationship_updater: ARU) -> Self {
        Self {
            account_relationship_updater,
        }
    }
}

impl<ARU> EventSaveHook<Account> for AccountEventSaveHook<ARU>
where
    ARU: AccountRelationshipUpdater,
{
    type Uow = ARU::Uow;
    type Error = AccountRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<AccountId, AccountEventPayload>,
    ) -> Result<(), Self::Error> {
        match event.payload() {
            AccountEventPayload::Opened { owner, .. } => {
                self.account_relationship_updater
                    .upsert_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            AccountEventPayload::OwnershipTransferred { owner } => {
                self.account_relationship_updater
                    .replace_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
