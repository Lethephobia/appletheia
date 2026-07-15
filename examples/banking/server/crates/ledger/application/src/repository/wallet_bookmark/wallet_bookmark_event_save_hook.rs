use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmark, WalletBookmarkEventPayload, WalletBookmarkId,
};

use crate::authorization::{
    WalletBookmarkRelationshipUpdater, WalletBookmarkRelationshipUpdaterError,
};

pub struct WalletBookmarkEventSaveHook<PRU>
where
    PRU: WalletBookmarkRelationshipUpdater,
{
    wallet_bookmark_relationship_updater: PRU,
}

impl<PRU> WalletBookmarkEventSaveHook<PRU>
where
    PRU: WalletBookmarkRelationshipUpdater,
{
    pub fn new(wallet_bookmark_relationship_updater: PRU) -> Self {
        Self {
            wallet_bookmark_relationship_updater,
        }
    }
}

impl<PRU> EventSaveHook<WalletBookmark> for WalletBookmarkEventSaveHook<PRU>
where
    PRU: WalletBookmarkRelationshipUpdater,
{
    type Uow = PRU::Uow;
    type Error = WalletBookmarkRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<WalletBookmarkId, WalletBookmarkEventPayload>,
    ) -> Result<(), Self::Error> {
        if let WalletBookmarkEventPayload::Registered { owner, .. } = event.payload() {
            self.wallet_bookmark_relationship_updater
                .upsert_owner(uow, event.aggregate_id(), *owner)
                .await?;
        }

        Ok(())
    }
}
