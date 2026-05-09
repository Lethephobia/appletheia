use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Projector specification for user-private information read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserPrivateInfoProjectorSpec;

impl ProjectorSpec for UserPrivateInfoProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_private_info"),
        Subscription::AnyOf(&[
            EventSelector::new(User::TYPE, UserEventPayload::REGISTERED),
            EventSelector::new(User::TYPE, UserEventPayload::IDENTITY_LINKED),
            EventSelector::new(User::TYPE, UserEventPayload::IDENTITY_EMAIL_CHANGED),
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
