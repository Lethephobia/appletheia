use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

#[aggregate_id]
pub struct TeamMembershipCleanupId(Uuid);

impl TeamMembershipCleanupId {
    /// Creates a new team membership cleanup ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TeamMembershipCleanupId {
    fn default() -> Self {
        Self::new()
    }
}
