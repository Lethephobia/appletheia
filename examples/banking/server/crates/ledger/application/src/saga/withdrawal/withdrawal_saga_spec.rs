use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalEventPayload};

use super::WithdrawalSagaState;

/// Declares the descriptor and state for the withdrawal saga.
pub struct WithdrawalSagaSpec;

impl SagaSpec for WithdrawalSagaSpec {
    type State = WithdrawalSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("withdrawal"),
        SagaStartEvents::new(&[EventSelector::new::<Withdrawal>(
            WithdrawalEventPayload::REQUESTED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::REQUESTED),
            EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVE_REJECTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::SETTLEMENT_EXECUTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::SETTLEMENT_EXECUTE_REJECTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::FAILED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASE_REJECTED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMITTED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMIT_REJECTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::COMPLETED),
        ]),
    );
}
