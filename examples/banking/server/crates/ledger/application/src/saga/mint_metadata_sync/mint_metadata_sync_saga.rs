use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{
    Currency, CurrencyEventPayload, MintMetadataSyncRejectionReason,
};

use super::{
    MintMetadataSyncSagaError, MintMetadataSyncSagaSpec, MintMetadataSyncSagaState,
    MintMetadataSyncSagaStatus,
};
use crate::command::MintMetadataSyncCommand;

/// Coordinates mint metadata synchronization after currency metadata changes.
pub struct MintMetadataSyncSaga;

impl Saga for MintMetadataSyncSaga {
    type Spec = MintMetadataSyncSagaSpec;
    type Error = MintMetadataSyncSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Currency>()?;

        match domain_event.payload() {
            CurrencyEventPayload::SymbolChanged { .. }
            | CurrencyEventPayload::NameChanged { .. }
            | CurrencyEventPayload::DescriptionChanged { .. }
            | CurrencyEventPayload::ImageChanged { .. } => {
                *instance.state_mut() =
                    Some(MintMetadataSyncSagaState::new(domain_event.aggregate_id()));
                instance.append_command(
                    event,
                    &MintMetadataSyncCommand {
                        currency_id: domain_event.aggregate_id(),
                    },
                )?;
                Ok(())
            }
            CurrencyEventPayload::MintMetadataSynced => {
                instance.state_required_mut()?.status = MintMetadataSyncSagaStatus::Synced;
                instance.succeed();
                Ok(())
            }
            CurrencyEventPayload::MintMetadataSyncRejected { reason } => {
                match reason {
                    MintMetadataSyncRejectionReason::NotProvisioned => {
                        instance.state_required_mut()?.status =
                            MintMetadataSyncSagaStatus::NotProvisioned;
                        instance.succeed();
                    }
                }
                Ok(())
            }
            _ => Err(MintMetadataSyncSagaError::UnexpectedEvent),
        }
    }
}
