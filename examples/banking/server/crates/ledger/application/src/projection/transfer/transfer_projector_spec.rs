use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

/// Declares the subscription for the transfer view projector.
pub struct TransferProjectorSpec;

impl ProjectorSpec for TransferProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("transfer"),
        Subscription::AnyOf(&[
            EventSelector::new(Transfer::TYPE, TransferEventPayload::REQUESTED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::COMPLETED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::FAILED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::CANCELLED),
        ]),
    );
}
