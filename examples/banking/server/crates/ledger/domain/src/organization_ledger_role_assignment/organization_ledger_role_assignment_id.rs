use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

#[aggregate_id]
pub struct OrganizationLedgerRoleAssignmentId(Uuid);

impl OrganizationLedgerRoleAssignmentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OrganizationLedgerRoleAssignmentId {
    fn default() -> Self {
        Self::new()
    }
}
