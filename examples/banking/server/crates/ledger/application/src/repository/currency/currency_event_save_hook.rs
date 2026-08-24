use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload, CurrencyId};

use crate::authorization::{CurrencyRelationshipUpdater, CurrencyRelationshipUpdaterError};

pub struct CurrencyEventSaveHook<RU>
where
    RU: CurrencyRelationshipUpdater,
{
    relationship_updater: RU,
}

impl<RU> CurrencyEventSaveHook<RU>
where
    RU: CurrencyRelationshipUpdater,
{
    pub fn new(relationship_updater: RU) -> Self {
        Self {
            relationship_updater,
        }
    }
}

impl<RU> EventSaveHook<Currency> for CurrencyEventSaveHook<RU>
where
    RU: CurrencyRelationshipUpdater,
{
    type Uow = RU::Uow;
    type Error = CurrencyRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<CurrencyId, CurrencyEventPayload>,
    ) -> Result<(), Self::Error> {
        if let CurrencyEventPayload::Defined {
            currency_registrar_id,
            ..
        } = event.payload()
        {
            self.relationship_updater
                .upsert_currency_registrar(uow, event.aggregate_id(), *currency_registrar_id)
                .await?;
        }

        Ok(())
    }
}
