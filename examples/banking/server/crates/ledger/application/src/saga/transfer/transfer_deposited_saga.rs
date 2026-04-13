use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    TransferDepositedSagaError, TransferDepositedSagaSpec, TransferSagaContext, TransferSagaStatus,
};
use crate::command::AccountCommitReservedFundsCommand;

/// Coordinates the transfer step after the destination account receives funds.
pub struct TransferDepositedSaga;

impl Saga for TransferDepositedSaga {
    type Spec = TransferDepositedSagaSpec;
    type Context = TransferSagaContext;
    type EventAggregate = Account;
    type Command = AccountCommitReservedFundsCommand;
    type Error = TransferDepositedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let AccountEventPayload::Deposited { .. } = event.payload() else {
            return Err(TransferDepositedSagaError::UnexpectedEvent);
        };
        let mut context = context.ok_or(TransferDepositedSagaError::ContextRequired)?;
        context.status = TransferSagaStatus::Deposited;

        let command = CommandRequest::new(AccountCommitReservedFundsCommand {
            account_id: context.from_account_id,
            amount: context.amount,
        });

        Ok(SagaTransition::new(context, command))
    }
}
