use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload, CurrencyId};

use crate::authorization::{CurrencyRelationshipUpdater, CurrencyRelationshipUpdaterError};

pub struct CurrencyEventSaveHook<CRU>
where
    CRU: CurrencyRelationshipUpdater,
{
    currency_relationship_updater: CRU,
}

impl<CRU> CurrencyEventSaveHook<CRU>
where
    CRU: CurrencyRelationshipUpdater,
{
    pub fn new(currency_relationship_updater: CRU) -> Self {
        Self {
            currency_relationship_updater,
        }
    }
}

impl<CRU> EventSaveHook<Currency> for CurrencyEventSaveHook<CRU>
where
    CRU: CurrencyRelationshipUpdater,
{
    type Uow = CRU::Uow;
    type Error = CurrencyRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<CurrencyId, CurrencyEventPayload>,
    ) -> Result<(), Self::Error> {
        match event.payload() {
            CurrencyEventPayload::Defined { owner, .. } => {
                self.currency_relationship_updater
                    .upsert_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                self.currency_relationship_updater
                    .replace_owner(uow, event.aggregate_id(), *owner)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
