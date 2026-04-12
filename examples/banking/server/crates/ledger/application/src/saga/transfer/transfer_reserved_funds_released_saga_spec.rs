use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::TransferFundsReservedSagaSpec;

pub struct TransferReservedFundsReleasedSagaSpec;

impl SagaSpec for TransferReservedFundsReleasedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer_reserved_funds_released"),
        EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_RELEASED),
        SagaPredecessor::Required(&TransferFundsReservedSagaSpec::DESCRIPTOR),
    );
}
