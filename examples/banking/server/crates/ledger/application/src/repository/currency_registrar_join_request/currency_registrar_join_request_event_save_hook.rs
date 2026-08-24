use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestEventPayload,
    CurrencyRegistrarJoinRequestId,
};

use crate::authorization::{
    CurrencyRegistrarJoinRequestRelationshipUpdater,
    CurrencyRegistrarJoinRequestRelationshipUpdaterError,
};

pub struct CurrencyRegistrarJoinRequestEventSaveHook<RU>
where
    RU: CurrencyRegistrarJoinRequestRelationshipUpdater,
{
    relationship_updater: RU,
}

impl<RU> CurrencyRegistrarJoinRequestEventSaveHook<RU>
where
    RU: CurrencyRegistrarJoinRequestRelationshipUpdater,
{
    pub fn new(relationship_updater: RU) -> Self {
        Self {
            relationship_updater,
        }
    }
}

impl<RU> EventSaveHook<CurrencyRegistrarJoinRequest>
    for CurrencyRegistrarJoinRequestEventSaveHook<RU>
where
    RU: CurrencyRegistrarJoinRequestRelationshipUpdater,
{
    type Uow = RU::Uow;
    type Error = CurrencyRegistrarJoinRequestRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<CurrencyRegistrarJoinRequestId, CurrencyRegistrarJoinRequestEventPayload>,
    ) -> Result<(), Self::Error> {
        if let CurrencyRegistrarJoinRequestEventPayload::Submitted {
            currency_registrar_id,
            requester_id,
        } = event.payload()
        {
            self.relationship_updater
                .upsert_registrar(uow, event.aggregate_id(), *currency_registrar_id)
                .await?;
            self.relationship_updater
                .upsert_requester(uow, event.aggregate_id(), *requester_id)
                .await?;
        }

        Ok(())
    }
}
