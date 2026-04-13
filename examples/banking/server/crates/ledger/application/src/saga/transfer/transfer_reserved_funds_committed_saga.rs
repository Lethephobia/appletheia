use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    TransferReservedFundsCommittedSagaError, TransferReservedFundsCommittedSagaSpec,
    TransferSagaContext, TransferSagaStatus,
};
use crate::command::TransferCompleteCommand;

/// Coordinates the transfer step after reserved funds are committed.
pub struct TransferReservedFundsCommittedSaga;

impl Saga for TransferReservedFundsCommittedSaga {
    type Spec = TransferReservedFundsCommittedSagaSpec;
    type Context = TransferSagaContext;
    type EventAggregate = Account;
    type Command = TransferCompleteCommand;
    type Error = TransferReservedFundsCommittedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let AccountEventPayload::ReservedFundsCommitted { .. } = event.payload() else {
            return Err(TransferReservedFundsCommittedSagaError::UnexpectedEvent);
        };
        let mut context =
            context.ok_or(TransferReservedFundsCommittedSagaError::ContextRequired)?;
        context.status = TransferSagaStatus::ReservedFundsCommitted;

        let command = CommandRequest::new(TransferCompleteCommand {
            transfer_id: context.transfer_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
