use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::payout_destination::{
    PayoutDestination, PayoutDestinationEventPayload, PayoutDestinationId,
};

use crate::authorization::{
    PayoutDestinationRelationshipUpdater, PayoutDestinationRelationshipUpdaterError,
};

pub struct PayoutDestinationEventSaveHook<PRU>
where
    PRU: PayoutDestinationRelationshipUpdater,
{
    payout_destination_relationship_updater: PRU,
}

impl<PRU> PayoutDestinationEventSaveHook<PRU>
where
    PRU: PayoutDestinationRelationshipUpdater,
{
    pub fn new(payout_destination_relationship_updater: PRU) -> Self {
        Self {
            payout_destination_relationship_updater,
        }
    }
}

impl<PRU> EventSaveHook<PayoutDestination> for PayoutDestinationEventSaveHook<PRU>
where
    PRU: PayoutDestinationRelationshipUpdater,
{
    type Uow = PRU::Uow;
    type Error = PayoutDestinationRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<PayoutDestinationId, PayoutDestinationEventPayload>,
    ) -> Result<(), Self::Error> {
        if let PayoutDestinationEventPayload::Registered { owner, .. } = event.payload() {
            self.payout_destination_relationship_updater
                .upsert_owner(uow, event.aggregate_id(), *owner)
                .await?;
        }

        Ok(())
    }
}
