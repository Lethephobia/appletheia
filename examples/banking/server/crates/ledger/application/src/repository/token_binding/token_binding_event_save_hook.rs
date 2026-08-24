use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_ledger_domain::token_binding::{
    TokenBinding, TokenBindingEventPayload, TokenBindingId,
};

use crate::authorization::{TokenBindingRelationshipUpdater, TokenBindingRelationshipUpdaterError};

pub struct TokenBindingEventSaveHook<RU>
where
    RU: TokenBindingRelationshipUpdater,
{
    relationship_updater: RU,
}

impl<RU> TokenBindingEventSaveHook<RU>
where
    RU: TokenBindingRelationshipUpdater,
{
    pub fn new(relationship_updater: RU) -> Self {
        Self {
            relationship_updater,
        }
    }
}

impl<RU> EventSaveHook<TokenBinding> for TokenBindingEventSaveHook<RU>
where
    RU: TokenBindingRelationshipUpdater,
{
    type Uow = RU::Uow;
    type Error = TokenBindingRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<TokenBindingId, TokenBindingEventPayload>,
    ) -> Result<(), Self::Error> {
        if let TokenBindingEventPayload::Defined { currency_id, .. } = event.payload() {
            self.relationship_updater
                .upsert_currency(uow, event.aggregate_id(), *currency_id)
                .await?;
        }
        Ok(())
    }
}
