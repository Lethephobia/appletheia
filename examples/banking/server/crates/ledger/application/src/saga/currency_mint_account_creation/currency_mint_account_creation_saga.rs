use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use crate::command::CurrencyMintAccountCreateCommand;

use super::{
    CurrencyMintAccountCreationSagaError, CurrencyMintAccountCreationSagaSpec,
    CurrencyMintAccountCreationSagaState,
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
        let event = event_envelope
            .try_into_domain_event::<Currency>()
            .map_err(|_| CurrencyMintAccountCreationSagaError::UnexpectedEvent)?;
        let CurrencyEventPayload::Defined { .. } = event.payload() else {
            return Err(CurrencyMintAccountCreationSagaError::UnexpectedEvent);
        };

        let state = CurrencyMintAccountCreationSagaState::new(event.aggregate_id());
        *instance.state_mut() = Some(state);
        instance
            .append_command(
                event_envelope,
                &CurrencyMintAccountCreateCommand {
                    currency_id: event.aggregate_id(),
                },
            )
            .map_err(|_| CurrencyMintAccountCreationSagaError::UnexpectedEvent)?;
        instance.succeed();

        Ok(())
    }
}
