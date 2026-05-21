use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{
    Currency, CurrencyEventPayload, CurrencyMintAccountCreationRequestRejectionReason,
    CurrencyMintAccountRecordRejectionReason,
};

use crate::command::{CurrencyMintAccountCreateCommand, CurrencyMintAccountRequestCommand};

use super::{
    CurrencyMintAccountCreationSagaError, CurrencyMintAccountCreationSagaSpec,
    CurrencyMintAccountCreationSagaState, CurrencyMintAccountCreationSagaStatus,
};

/// Coordinates mint account creation after a currency is defined.
pub struct CurrencyMintAccountCreationSaga;

impl Saga for CurrencyMintAccountCreationSaga {
    type Spec = CurrencyMintAccountCreationSagaSpec;
    type Error = CurrencyMintAccountCreationSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope.try_into_domain_event::<Currency>()?;
        match event.payload() {
            CurrencyEventPayload::Defined { .. } => {
                *instance.state_mut() = Some(CurrencyMintAccountCreationSagaState::new(
                    event.aggregate_id(),
                ));
                instance.append_command(
                    event_envelope,
                    &CurrencyMintAccountRequestCommand {
                        currency_id: event.aggregate_id(),
                    },
                )?;
            }
            CurrencyEventPayload::MintAccountCreationRequested => {
                let state = instance.state_required_mut()?;
                state.status = CurrencyMintAccountCreationSagaStatus::RequestPersisted;
                instance.append_command(
                    event_envelope,
                    &CurrencyMintAccountCreateCommand {
                        currency_id: event.aggregate_id(),
                    },
                )?;
            }
            CurrencyEventPayload::MintAccountRecorded { .. } => {
                instance.state_required_mut()?.status =
                    CurrencyMintAccountCreationSagaStatus::Completed;
                instance.succeed();
            }
            CurrencyEventPayload::MintAccountCreationRequestRejected { reason } => match reason {
                CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested => {
                    let state = instance.state_required_mut()?;
                    state.status = CurrencyMintAccountCreationSagaStatus::RequestPersisted;
                    instance.append_command(
                        event_envelope,
                        &CurrencyMintAccountCreateCommand {
                            currency_id: event.aggregate_id(),
                        },
                    )?;
                }
                CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded => {
                    instance.state_required_mut()?.status =
                        CurrencyMintAccountCreationSagaStatus::Completed;
                    instance.succeed();
                }
                CurrencyMintAccountCreationRequestRejectionReason::Removed => {
                    instance.state_required_mut()?.status =
                        CurrencyMintAccountCreationSagaStatus::Failed;
                    instance.fail();
                }
            },
            CurrencyEventPayload::MintAccountRecordRejected { reason, .. } => match reason {
                CurrencyMintAccountRecordRejectionReason::AlreadyRecorded => {
                    instance.state_required_mut()?.status =
                        CurrencyMintAccountCreationSagaStatus::Completed;
                    instance.succeed();
                }
                CurrencyMintAccountRecordRejectionReason::Removed => {
                    instance.state_required_mut()?.status =
                        CurrencyMintAccountCreationSagaStatus::Failed;
                    instance.fail();
                }
            },
            _ => return Err(CurrencyMintAccountCreationSagaError::UnexpectedEvent),
        }

        Ok(())
    }
}
