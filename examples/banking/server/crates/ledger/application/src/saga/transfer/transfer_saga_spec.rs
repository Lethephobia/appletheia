use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::account::Account;
use banking_ledger_domain::account::AccountEventPayload;
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::TransferSagaState;

/// Declares the descriptor and state for the transfer saga.
pub struct TransferSagaSpec;

impl SagaSpec for TransferSagaSpec {
    type State = TransferSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer"),
        SagaStartEvents::new(&[EventSelector::new::<Transfer>(
            TransferEventPayload::REQUESTED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<Transfer>(TransferEventPayload::REQUESTED),
            EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVE_REJECTED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSITED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSIT_REJECTED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASE_REJECTED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMITTED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMIT_REJECTED),
            EventSelector::new::<Account>(AccountEventPayload::WITHDRAWN),
            EventSelector::new::<Account>(AccountEventPayload::WITHDRAW_REJECTED),
            EventSelector::new::<Transfer>(TransferEventPayload::COMPLETED),
            EventSelector::new::<Transfer>(TransferEventPayload::FAILED),
        ]),
    );
}
