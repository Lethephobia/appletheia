use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{
    Currency, CurrencyEventPayload, CurrencyMintAccountMetadataSyncRejectionReason,
};

use super::{
    CurrencyMintAccountMetadataSyncSagaError, CurrencyMintAccountMetadataSyncSagaSpec,
    CurrencyMintAccountMetadataSyncSagaState,
};
use crate::command::CurrencyMintAccountMetadataSyncCommand;

/// Coordinates mint metadata synchronization after currency metadata changes.
pub struct CurrencyMintAccountMetadataSyncSaga;

impl Saga for CurrencyMintAccountMetadataSyncSaga {
    type Spec = CurrencyMintAccountMetadataSyncSagaSpec;
    type Error = CurrencyMintAccountMetadataSyncSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope.try_into_domain_event::<Currency>()?;

        match event.payload() {
            CurrencyEventPayload::SymbolChanged { .. }
            | CurrencyEventPayload::NameChanged { .. }
            | CurrencyEventPayload::DescriptionChanged { .. }
            | CurrencyEventPayload::ImageChanged { .. } => {
                *instance.state_mut() = Some(CurrencyMintAccountMetadataSyncSagaState::new(
                    event.aggregate_id(),
                ));
                instance.append_command(
                    event_envelope,
                    &CurrencyMintAccountMetadataSyncCommand {
                        currency_id: event.aggregate_id(),
                    },
                )?;
                Ok(())
            }
            CurrencyEventPayload::MintAccountMetadataSynced => {
                instance.succeed();
                Ok(())
            }
            CurrencyEventPayload::MintAccountMetadataSyncRejected { reason } => {
                match reason {
                    CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned => {
                        instance.succeed();
                    }
                }
                Ok(())
            }
            _ => Err(CurrencyMintAccountMetadataSyncSagaError::UnexpectedEvent),
        }
    }
}
