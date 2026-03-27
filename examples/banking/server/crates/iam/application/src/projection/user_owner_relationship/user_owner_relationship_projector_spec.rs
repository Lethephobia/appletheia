use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the subscription for the user owner relationship projector.
pub struct UserOwnerRelationshipProjectorSpec;

impl ProjectorSpec for UserOwnerRelationshipProjectorSpec {
    const NAME: ProjectorName = ProjectorName::new("user_owner_relationship");
    const SUBSCRIPTION: Subscription<'static, EventSelector> =
        Subscription::Only(&[EventSelector::new(User::TYPE, UserEventPayload::REGISTERED)]);
}
