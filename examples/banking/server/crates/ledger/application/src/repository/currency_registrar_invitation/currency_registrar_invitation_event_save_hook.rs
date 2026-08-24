use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::{
    CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload,
    CurrencyRegistrarInvitationId,
};

use crate::authorization::{
    CurrencyRegistrarInvitationRelationshipUpdater,
    CurrencyRegistrarInvitationRelationshipUpdaterError,
};

pub struct CurrencyRegistrarInvitationEventSaveHook<RU>
where
    RU: CurrencyRegistrarInvitationRelationshipUpdater,
{
    relationship_updater: RU,
}

impl<RU> CurrencyRegistrarInvitationEventSaveHook<RU>
where
    RU: CurrencyRegistrarInvitationRelationshipUpdater,
{
    pub fn new(relationship_updater: RU) -> Self {
        Self {
            relationship_updater,
        }
    }
}

impl<RU> EventSaveHook<CurrencyRegistrarInvitation> for CurrencyRegistrarInvitationEventSaveHook<RU>
where
    RU: CurrencyRegistrarInvitationRelationshipUpdater,
{
    type Uow = RU::Uow;
    type Error = CurrencyRegistrarInvitationRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<CurrencyRegistrarInvitationId, CurrencyRegistrarInvitationEventPayload>,
    ) -> Result<(), Self::Error> {
        if let CurrencyRegistrarInvitationEventPayload::Issued {
            currency_registrar_id,
            invitee_id,
            ..
        } = event.payload()
        {
            self.relationship_updater
                .upsert_registrar(uow, event.aggregate_id(), *currency_registrar_id)
                .await?;
            self.relationship_updater
                .upsert_invitee(uow, event.aggregate_id(), *invitee_id)
                .await?;
        }

        Ok(())
    }
}
