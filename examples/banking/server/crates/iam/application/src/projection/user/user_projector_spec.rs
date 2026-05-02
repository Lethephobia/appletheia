use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the subscription for the user view projector.
pub struct UserProjectorSpec;

impl ProjectorSpec for UserProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user"),
        Subscription::AnyOf(&[
            EventSelector::new(User::TYPE, UserEventPayload::REGISTERED),
            EventSelector::new(User::TYPE, UserEventPayload::USERNAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::BIO_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::ACTIVATED),
            EventSelector::new(User::TYPE, UserEventPayload::INACTIVATED),
            EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
        ]),
    );
}
