use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{UserRoleAssignment, UserRoleAssignmentEventPayload};

/// Defines the stable name for the role assignee relationship projector.
pub struct RoleAssigneeRelationshipProjectorSpec;

impl ProjectorSpec for RoleAssigneeRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("role_assignee_relationship"),
        Subscription::Only(&[
            EventSelector::new(
                UserRoleAssignment::TYPE,
                UserRoleAssignmentEventPayload::ASSIGNED,
            ),
            EventSelector::new(
                UserRoleAssignment::TYPE,
                UserRoleAssignmentEventPayload::REVOKED,
            ),
        ]),
    );
}
