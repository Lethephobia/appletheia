use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{UserRoleAssignment, UserRoleAssignmentEventPayload};

/// Defines the stable name for the role assignee relationship projector.
pub struct RoleAssigneeRelationshipProjectorSpec;

impl ProjectorSpec for RoleAssigneeRelationshipProjectorSpec {
    const NAME: ProjectorName = ProjectorName::new("role_assignee_relationship");
    const SUBSCRIPTION: Subscription<'static, EventSelector> = Subscription::Only(&[
        EventSelector::new(
            UserRoleAssignment::TYPE,
            UserRoleAssignmentEventPayload::ASSIGNED,
        ),
        EventSelector::new(
            UserRoleAssignment::TYPE,
            UserRoleAssignmentEventPayload::REVOKED,
        ),
    ]);
}
