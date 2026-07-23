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
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Currency>()?;
        match domain_event.payload() {
            CurrencyEventPayload::Defined { .. } => {
                *instance.state_mut() = Some(CurrencyProvisioningSagaState::new(
                    domain_event.aggregate_id(),
                ));
                instance.state_required_mut()?.status =
                    CurrencyProvisioningSagaStatus::ProvisionRequested;

                instance.append_command(
                    event,
                    &CurrencyProvisionCommand {
                        currency_id: domain_event.aggregate_id(),
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
