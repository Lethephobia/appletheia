use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{User, UserEventPayload};

/// Projector specification for public user profile read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserPublicProfileProjectorSpec;

impl ProjectorSpec for UserPublicProfileProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_public_profile"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::BIO_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ACTIVATED),
            EventSelector::new::<User>(UserEventPayload::INACTIVATED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}
