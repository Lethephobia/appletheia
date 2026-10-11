use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

#[aggregate_id]
pub struct OrganizationRoleAssignmentId(Uuid);

impl OrganizationRoleAssignmentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationRoleAssignmentId {
    fn default() -> Self {
        Self::new()
    }
}
