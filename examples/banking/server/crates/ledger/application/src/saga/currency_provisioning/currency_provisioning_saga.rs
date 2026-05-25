use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{
    Currency, CurrencyEventPayload, CurrencyProvisionRejectionReason,
};

use crate::command::CurrencyProvisionCommand;

use super::{
    CurrencyProvisioningSagaError, CurrencyProvisioningSagaSpec, CurrencyProvisioningSagaState,
    CurrencyProvisioningSagaStatus,
};

/// Coordinates currency provisioning after a currency is defined.
pub struct CurrencyProvisioningSaga;

impl Saga for CurrencyProvisioningSaga {
    type Spec = CurrencyProvisioningSagaSpec;
    type Error = CurrencyProvisioningSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope.try_into_domain_event::<Currency>()?;
        match event.payload() {
            CurrencyEventPayload::Defined { .. } => {
                *instance.state_mut() =
                    Some(CurrencyProvisioningSagaState::new(event.aggregate_id()));
                instance.append_command(
                    event_envelope,
                    &CurrencyProvisionCommand {
                        currency_id: event.aggregate_id(),
                    },
                )?;
            }
            CurrencyEventPayload::Provisioned { .. } => {
                instance.state_required_mut()?.status = CurrencyProvisioningSagaStatus::Completed;
                instance.succeed();
            }
            CurrencyEventPayload::ProvisionRejected { reason, .. } => match reason {
                CurrencyProvisionRejectionReason::AlreadyProvisioned => {
                    instance.state_required_mut()?.status =
                        CurrencyProvisioningSagaStatus::Completed;
                    instance.succeed();
                }
                CurrencyProvisionRejectionReason::Removed => {
                    instance.state_required_mut()?.status = CurrencyProvisioningSagaStatus::Failed;
                    instance.fail();
                }
            },
            _ => return Err(CurrencyProvisioningSagaError::UnexpectedEvent),
        }

        Ok(())
    }
}
