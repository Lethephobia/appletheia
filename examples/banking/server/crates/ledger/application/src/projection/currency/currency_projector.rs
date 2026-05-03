use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload, CurrencyStatus};

use super::{CurrencyProjectorError, CurrencyProjectorSpec};
use crate::view::{CurrencyViewStore, CurrencyViewUpsert};

/// Projects currency events into normalized currency views.
pub struct CurrencyProjector<VS>
where
    VS: CurrencyViewStore,
{
    view_store: VS,
}

impl<VS> CurrencyProjector<VS>
where
    VS: CurrencyViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for CurrencyProjector<VS>
where
    VS: CurrencyViewStore,
{
    type Spec = CurrencyProjectorSpec;
    type Uow = VS::Uow;
    type Error = CurrencyProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Currency>()?;
        let currency_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyEventPayload::Defined {
                owner,
                symbol,
                name,
                decimals,
                ..
            } => {
                self.view_store
                    .upsert(
                        uow,
                        CurrencyViewUpsert {
                            id: currency_id,
                            owner: *owner,
                            symbol: symbol.clone(),
                            name: name.clone(),
                            decimals: *decimals,
                            supply: CurrencyAmount::zero(),
                            status: CurrencyStatus::Active,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                self.view_store
                    .update_owner(uow, currency_id, *owner, event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.view_store
                    .update_symbol(uow, currency_id, symbol.clone(), event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::NameChanged { name } => {
                self.view_store
                    .update_name(uow, currency_id, name.clone(), event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::SupplyIncreased { amount } => {
                self.view_store
                    .increase_supply(uow, currency_id, *amount, event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::SupplyDecreased { amount } => {
                self.view_store
                    .decrease_supply(uow, currency_id, *amount, event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::Activated => {
                self.view_store
                    .update_status(
                        uow,
                        currency_id,
                        CurrencyStatus::Active,
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyEventPayload::Deactivated => {
                self.view_store
                    .update_status(
                        uow,
                        currency_id,
                        CurrencyStatus::Inactive,
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyEventPayload::Removed => {
                self.view_store
                    .delete(uow, currency_id, event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::SupplyIncreaseRejected { .. }
            | CurrencyEventPayload::SupplyDecreaseRejected { .. }
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
