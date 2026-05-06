use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

use super::UserPictureSagaState;

/// Declares the descriptor and state for the user picture saga.
pub struct UserPictureChangedSagaSpec;

impl SagaSpec for UserPictureChangedSagaSpec {
    type State = UserPictureSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("user_picture_changed"),
        EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
        Subscription::One(&EventSelector::new(
            User::TYPE,
            UserEventPayload::PICTURE_CHANGED,
        )),
    );
}
