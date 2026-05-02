use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the subscription for the user identity view projector.
pub struct UserIdentityProjectorSpec;

impl ProjectorSpec for UserIdentityProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_identity"),
        Subscription::AnyOf(&[
            EventSelector::new(User::TYPE, UserEventPayload::IDENTITY_LINKED),
            EventSelector::new(User::TYPE, UserEventPayload::IDENTITY_EMAIL_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
        ]),
    );
}
