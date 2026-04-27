use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the descriptor and state for the user picture saga.
pub struct UserPictureChangedSagaSpec;

impl SagaSpec for UserPictureChangedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("user_picture_changed"),
        EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
        SagaPredecessor::None,
    );
}
