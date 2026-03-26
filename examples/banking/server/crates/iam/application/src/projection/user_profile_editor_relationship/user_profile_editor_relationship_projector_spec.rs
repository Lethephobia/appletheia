use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};

/// Defines the stable name for the user profile editor relationship projector.
pub struct UserProfileEditorRelationshipProjectorSpec;

impl ProjectorSpec for UserProfileEditorRelationshipProjectorSpec {
    const NAME: ProjectorName = ProjectorName::new("user_profile_editor_relationship");
    const SUBSCRIPTION: Subscription<'static, EventSelector> = Subscription::Only(&[
        EventSelector::new(User::TYPE, UserEventPayload::REGISTERED),
        EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
    ]);
}
