use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    CurrencyIssuanceDepositedSagaError, CurrencyIssuanceDepositedSagaSpec,
    CurrencyIssuanceSagaContext, CurrencyIssuanceSagaStatus,
};
use crate::command::CurrencyIssuanceCompleteCommand;

/// Coordinates the currency issuance step after funds are deposited.
pub struct CurrencyIssuanceDepositedSaga;

impl Saga for CurrencyIssuanceDepositedSaga {
    type Spec = CurrencyIssuanceDepositedSagaSpec;
    type Context = CurrencyIssuanceSagaContext;
    type EventAggregate = Account;
    type Command = CurrencyIssuanceCompleteCommand;
    type Error = CurrencyIssuanceDepositedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let AccountEventPayload::Deposited { .. } = event.payload() else {
            return Err(CurrencyIssuanceDepositedSagaError::UnexpectedEvent);
        };
        let mut context = context.ok_or(CurrencyIssuanceDepositedSagaError::ContextRequired)?;
        context.status = CurrencyIssuanceSagaStatus::Deposited;

        let command = CommandRequest::new(CurrencyIssuanceCompleteCommand {
            currency_issuance_id: context.currency_issuance_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
