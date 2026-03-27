use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Declares the subscription for the user status manager relationship projector.
pub struct UserStatusManagerRelationshipProjectorSpec;

impl ProjectorSpec for UserStatusManagerRelationshipProjectorSpec {
    const NAME: ProjectorName = ProjectorName::new("user_status_manager_relationship");
    const SUBSCRIPTION: Subscription<'static, EventSelector> =
        Subscription::Only(&[EventSelector::new(User::TYPE, UserEventPayload::REGISTERED)]);
}
