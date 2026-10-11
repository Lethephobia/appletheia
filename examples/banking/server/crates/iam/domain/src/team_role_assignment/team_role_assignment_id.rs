use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

#[aggregate_id]
pub struct TeamRoleAssignmentId(Uuid);

impl TeamRoleAssignmentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TeamRoleAssignmentId {
    fn default() -> Self {
        Self::new()
    }
}
