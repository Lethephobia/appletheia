use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
    CurrencyRegistrarMembershipId,
};

use crate::authorization::{
    CurrencyRegistrarMembershipRelationshipUpdater,
    CurrencyRegistrarMembershipRelationshipUpdaterError,
};

pub struct CurrencyRegistrarMembershipEventSaveHook<RU>
where
    RU: CurrencyRegistrarMembershipRelationshipUpdater,
{
    relationship_updater: RU,
}

impl<RU> CurrencyRegistrarMembershipEventSaveHook<RU>
where
    RU: CurrencyRegistrarMembershipRelationshipUpdater,
{
    pub fn new(relationship_updater: RU) -> Self {
        Self {
            relationship_updater,
        }
    }
}

impl<RU> EventSaveHook<CurrencyRegistrarMembership> for CurrencyRegistrarMembershipEventSaveHook<RU>
where
    RU: CurrencyRegistrarMembershipRelationshipUpdater,
{
    type Uow = RU::Uow;
    type Error = CurrencyRegistrarMembershipRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<CurrencyRegistrarMembershipId, CurrencyRegistrarMembershipEventPayload>,
    ) -> Result<(), Self::Error> {
        match event.payload() {
            CurrencyRegistrarMembershipEventPayload::Created {
                currency_registrar_id,
                user_id,
            } => {
                self.relationship_updater
                    .upsert_registrar(uow, event.aggregate_id(), *currency_registrar_id)
                    .await?;
                self.relationship_updater
                    .upsert_member(uow, *currency_registrar_id, *user_id)
                    .await?;
            }
            CurrencyRegistrarMembershipEventPayload::Removed {
                currency_registrar_id,
                user_id,
            } => {
                self.relationship_updater
                    .remove_member(uow, *currency_registrar_id, *user_id)
                    .await?;
            }
            CurrencyRegistrarMembershipEventPayload::CreateRejected { .. }
            | CurrencyRegistrarMembershipEventPayload::RemoveRejected { .. } => {}
        }
        Ok(())
    }
}
