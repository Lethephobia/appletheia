use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::{
    CurrencyIssuanceIssuedSagaError, CurrencyIssuanceIssuedSagaSpec, CurrencyIssuanceSagaContext,
};
use crate::command::{CurrencyDefinitionIncreaseSupplyCommand, CurrencyIssuanceFailCommand};

/// Coordinates the first currency issuance step.
pub struct CurrencyIssuanceIssuedSaga;

impl Saga for CurrencyIssuanceIssuedSaga {
    type Spec = CurrencyIssuanceIssuedSagaSpec;
    type Context = CurrencyIssuanceSagaContext;
    type EventAggregate = CurrencyIssuance;
    type Command = CurrencyDefinitionIncreaseSupplyCommand;
    type Error = CurrencyIssuanceIssuedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let CurrencyIssuanceEventPayload::Issued {
            id,
            currency_definition_id,
            destination_account_id,
            amount,
        } = event.payload()
        else {
            return Err(CurrencyIssuanceIssuedSagaError::UnexpectedEvent);
        };

        let context = CurrencyIssuanceSagaContext::new(
            *id,
            *currency_definition_id,
            *destination_account_id,
            *amount,
        );

        let command = CommandRequest::with_failure_follow_up(
            CurrencyDefinitionIncreaseSupplyCommand {
                currency_definition_id: *currency_definition_id,
                amount: *amount,
            },
            CommandRequest::new(CurrencyIssuanceFailCommand {
                currency_issuance_id: *id,
            }),
        )
        .map_err(CurrencyIssuanceIssuedSagaError::from)?;

        Ok(SagaTransition::new(context, command))
    }
}
