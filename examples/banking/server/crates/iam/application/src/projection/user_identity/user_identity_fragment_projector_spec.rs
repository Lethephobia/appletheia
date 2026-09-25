use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{User, UserEventPayload};

/// Projector specification for user identity fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserIdentityFragmentProjectorSpec;

impl ProjectorSpec for UserIdentityFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_identity_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::IDENTITY_LINKED),
            EventSelector::new::<User>(UserEventPayload::IDENTITY_EMAIL_CHANGED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}
