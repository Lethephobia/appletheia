use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_iam_domain::{User, UserEventPayload};


/// Declares the descriptor for the user old picture object deletion saga.
pub struct UserOldPictureObjectDeletionSagaSpec;

impl SagaSpec for UserOldPictureObjectDeletionSagaSpec {

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("user_old_picture_object_deletion"),
        SagaStartEvents::new(&[EventSelector::new::<User>(
            UserEventPayload::PICTURE_CHANGED,
        )]),
        Subscription::One(&EventSelector::new::<User>(
            UserEventPayload::PICTURE_CHANGED,
        )),
    );
}
