use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{User, UserEventPayload};

/// Projector specification for public user list read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicUserListProjectorSpec;

impl ProjectorSpec for PublicUserListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("public_user_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ACTIVATED),
            EventSelector::new::<User>(UserEventPayload::DEACTIVATED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}
