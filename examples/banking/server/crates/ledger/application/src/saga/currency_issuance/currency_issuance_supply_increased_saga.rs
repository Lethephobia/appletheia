use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{
    CurrencyIssuanceSagaContext, CurrencyIssuanceSagaStatus,
    CurrencyIssuanceSupplyIncreasedSagaError, CurrencyIssuanceSupplyIncreasedSagaSpec,
};
use crate::command::{
    AccountDepositCommand, CurrencyDecreaseSupplyCommand, CurrencyIssuanceFailCommand,
};

/// Coordinates the currency issuance step after supply is increased.
pub struct CurrencyIssuanceSupplyIncreasedSaga;

impl Saga for CurrencyIssuanceSupplyIncreasedSaga {
    type Spec = CurrencyIssuanceSupplyIncreasedSagaSpec;
    type Context = CurrencyIssuanceSagaContext;
    type EventAggregate = Currency;
    type Command = AccountDepositCommand;
    type Error = CurrencyIssuanceSupplyIncreasedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let CurrencyEventPayload::SupplyIncreased { .. } = event.payload() else {
            return Err(CurrencyIssuanceSupplyIncreasedSagaError::UnexpectedEvent);
        };
        let mut context =
            context.ok_or(CurrencyIssuanceSupplyIncreasedSagaError::ContextRequired)?;
        context.status = CurrencyIssuanceSagaStatus::SupplyIncreased;

        let command = CommandRequest::with_failure_follow_up(
            AccountDepositCommand {
                account_id: context.destination_account_id,
                amount: context.amount,
            },
            CommandRequest::with_failure_follow_up(
                CurrencyDecreaseSupplyCommand {
                    currency_id: context.currency_id,
                    amount: context.amount,
                },
                CommandRequest::new(CurrencyIssuanceFailCommand {
                    currency_issuance_id: context.currency_issuance_id,
                }),
            )?,
        )
        .map_err(CurrencyIssuanceSupplyIncreasedSagaError::from)?;

        Ok(SagaTransition::new(context, command))
    }
}
