use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

/// Declares the descriptor and state for the transfer saga.
pub struct TransferRequestedSagaSpec;

impl SagaSpec for TransferRequestedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer_requested"),
        EventSelector::new(Transfer::TYPE, TransferEventPayload::REQUESTED),
        SagaPredecessor::None,
    );
}
