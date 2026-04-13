use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::TransferDepositedSagaSpec;

pub struct TransferReservedFundsCommittedSagaSpec;

impl SagaSpec for TransferReservedFundsCommittedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer_reserved_funds_committed"),
        EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_COMMITTED),
        SagaPredecessor::Required(&TransferDepositedSagaSpec::DESCRIPTOR),
    );
}
