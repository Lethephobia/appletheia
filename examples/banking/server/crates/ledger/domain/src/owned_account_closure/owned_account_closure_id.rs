use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `OwnedAccountClosure` aggregate.
#[aggregate_id]
pub struct OwnedAccountClosureId(Uuid);

impl OwnedAccountClosureId {
    /// Creates a new owned account closure ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OwnedAccountClosureId {
    fn default() -> Self {
        Self::new()
    }
}
