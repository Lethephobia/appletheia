use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{
    CurrencyIssuanceSagaContext, CurrencyIssuanceSagaStatus,
    CurrencyIssuanceSupplyDecreasedSagaError, CurrencyIssuanceSupplyDecreasedSagaSpec,
};
use crate::command::CurrencyIssuanceFailCommand;

/// Coordinates the currency issuance compensation step after supply is decreased.
pub struct CurrencyIssuanceSupplyDecreasedSaga;

impl Saga for CurrencyIssuanceSupplyDecreasedSaga {
    type Spec = CurrencyIssuanceSupplyDecreasedSagaSpec;
    type Context = CurrencyIssuanceSagaContext;
    type EventAggregate = Currency;
    type Command = CurrencyIssuanceFailCommand;
    type Error = CurrencyIssuanceSupplyDecreasedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let CurrencyEventPayload::SupplyDecreased { .. } = event.payload() else {
            return Err(CurrencyIssuanceSupplyDecreasedSagaError::UnexpectedEvent);
        };
        let mut context =
            context.ok_or(CurrencyIssuanceSupplyDecreasedSagaError::ContextRequired)?;
        context.status = CurrencyIssuanceSagaStatus::SupplyDecreased;

        let command = CommandRequest::new(CurrencyIssuanceFailCommand {
            currency_issuance_id: context.currency_issuance_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
