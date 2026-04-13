use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    TransferFundsReservedSagaError, TransferFundsReservedSagaSpec, TransferSagaContext,
    TransferSagaStatus,
};
use crate::command::{
    AccountDepositCommand, AccountReleaseReservedFundsCommand, TransferFailCommand,
};

/// Coordinates the transfer step after the source account reserves funds.
pub struct TransferFundsReservedSaga;

impl Saga for TransferFundsReservedSaga {
    type Spec = TransferFundsReservedSagaSpec;
    type Context = TransferSagaContext;
    type EventAggregate = Account;
    type Command = AccountDepositCommand;
    type Error = TransferFundsReservedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let AccountEventPayload::FundsReserved { .. } = event.payload() else {
            return Err(TransferFundsReservedSagaError::UnexpectedEvent);
        };
        let mut context = context.ok_or(TransferFundsReservedSagaError::ContextRequired)?;
        context.status = TransferSagaStatus::FundsReserved;

        let command = CommandRequest::with_failure_follow_up(
            AccountDepositCommand {
                account_id: context.to_account_id,
                amount: context.amount,
            },
            CommandRequest::with_failure_follow_up(
                AccountReleaseReservedFundsCommand {
                    account_id: context.from_account_id,
                    amount: context.amount,
                },
                CommandRequest::new(TransferFailCommand {
                    transfer_id: context.transfer_id,
                }),
            )?,
        )
        .map_err(TransferFundsReservedSagaError::from)?;

        Ok(SagaTransition::new(context, command))
    }
}
