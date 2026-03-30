use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::TransferSagaState;

/// Declares the descriptor and state for the transfer saga.
pub struct TransferSagaSpec;

impl SagaSpec for TransferSagaSpec {
    type State = TransferSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer"),
        Subscription::Only(&[
            EventSelector::new(Account::TYPE, AccountEventPayload::TRANSFER_REQUESTED),
            EventSelector::new(Account::TYPE, AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_COMMITTED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::INITIATED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::COMPLETED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::FAILED),
        ]),
    );
}
