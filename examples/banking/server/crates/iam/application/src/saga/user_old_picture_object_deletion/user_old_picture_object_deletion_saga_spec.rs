use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

use super::UserOldPictureObjectDeletionSagaState;

/// Declares the descriptor and state for the user old picture object deletion saga.
pub struct UserOldPictureObjectDeletionSagaSpec;

impl SagaSpec for UserOldPictureObjectDeletionSagaSpec {
    type State = UserOldPictureObjectDeletionSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("user_old_picture_object_deletion"),
        EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
        Subscription::One(&EventSelector::new(
            User::TYPE,
            UserEventPayload::PICTURE_CHANGED,
        )),
    );
}
