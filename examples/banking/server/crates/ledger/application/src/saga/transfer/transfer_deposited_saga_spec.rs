use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::TransferFundsReservedSagaSpec;

pub struct TransferDepositedSagaSpec;

impl SagaSpec for TransferDepositedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer_deposited"),
        EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
        SagaPredecessor::Required(&TransferFundsReservedSagaSpec::DESCRIPTOR),
    );
}
