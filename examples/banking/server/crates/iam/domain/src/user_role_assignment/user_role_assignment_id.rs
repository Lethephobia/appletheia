use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `UserRoleAssignment` aggregate.
#[aggregate_id]
pub struct UserRoleAssignmentId(Uuid);

impl UserRoleAssignmentId {
    /// Creates a new assignment ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for UserRoleAssignmentId {
    fn default() -> Self {
        Self::new()
    }
}
