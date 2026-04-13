use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::{
    TransferReservedFundsReleasedSagaError, TransferReservedFundsReleasedSagaSpec,
    TransferSagaContext, TransferSagaStatus,
};
use crate::command::TransferFailCommand;

/// Coordinates the transfer compensation step after reserved funds are released.
pub struct TransferReservedFundsReleasedSaga;

impl Saga for TransferReservedFundsReleasedSaga {
    type Spec = TransferReservedFundsReleasedSagaSpec;
    type Context = TransferSagaContext;
    type EventAggregate = Account;
    type Command = TransferFailCommand;
    type Error = TransferReservedFundsReleasedSagaError;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let AccountEventPayload::ReservedFundsReleased { .. } = event.payload() else {
            return Err(TransferReservedFundsReleasedSagaError::UnexpectedEvent);
        };
        let mut context = context.ok_or(TransferReservedFundsReleasedSagaError::ContextRequired)?;
        context.status = TransferSagaStatus::ReservedFundsReleased;

        let command = CommandRequest::new(TransferFailCommand {
            transfer_id: context.transfer_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
