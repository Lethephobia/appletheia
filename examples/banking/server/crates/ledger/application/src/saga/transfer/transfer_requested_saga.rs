use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::{TransferRequestedSagaError, TransferRequestedSagaSpec, TransferSagaContext};
use crate::command::{AccountReserveFundsCommand, TransferFailCommand};

/// Coordinates the first transfer step.
pub struct TransferRequestedSaga;

impl Saga for TransferRequestedSaga {
    type Spec = TransferRequestedSagaSpec;
    type Context = TransferSagaContext;
    type EventAggregate = Transfer;
    type Command = AccountReserveFundsCommand;
    type Error = TransferRequestedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let TransferEventPayload::Requested {
            id,
            from_account_id,
            to_account_id,
            amount,
        } = event.payload()
        else {
            return Err(TransferRequestedSagaError::UnexpectedEvent);
        };

        let context = TransferSagaContext::new(*id, *from_account_id, *to_account_id, *amount);

        let command = CommandRequest::with_failure_follow_up(
            AccountReserveFundsCommand {
                account_id: *from_account_id,
                amount: *amount,
            },
            CommandRequest::new(TransferFailCommand { transfer_id: *id }),
        )
        .map_err(TransferRequestedSagaError::from)?;

        Ok(SagaTransition::new(context, command))
    }
}
