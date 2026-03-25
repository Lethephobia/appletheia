use appletheia::application::projection::ProjectorName;

/// Defines the stable name for the role assignee relationship projector.
pub struct RoleAssigneeRelationshipProjector;

impl RoleAssigneeRelationshipProjector {
    pub const NAME: ProjectorName = ProjectorName::new("role_assignee_relationship");
}
