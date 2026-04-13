use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the subscription for the user owner relationship projector.
pub struct UserOwnerRelationshipProjectorSpec;

impl ProjectorSpec for UserOwnerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_owner_relationship"),
        Subscription::AnyOf(&[EventSelector::new(User::TYPE, UserEventPayload::REGISTERED)]),
    );
}
