use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationEventPayload};

/// Declares the descriptor and state for the organization picture saga.
pub struct OrganizationPictureChangedSagaSpec;

impl SagaSpec for OrganizationPictureChangedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_picture_changed"),
        EventSelector::new(
            Organization::TYPE,
            OrganizationEventPayload::PICTURE_CHANGED,
        ),
        SagaPredecessor::None,
    );
}
