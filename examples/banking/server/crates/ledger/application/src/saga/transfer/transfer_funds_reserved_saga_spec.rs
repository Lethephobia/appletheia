use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::TransferRequestedSagaSpec;

pub struct TransferFundsReservedSagaSpec;

impl SagaSpec for TransferFundsReservedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer_funds_reserved"),
        EventSelector::new(Account::TYPE, AccountEventPayload::FUNDS_RESERVED),
        SagaPredecessor::Required(&TransferRequestedSagaSpec::DESCRIPTOR),
    );
}
