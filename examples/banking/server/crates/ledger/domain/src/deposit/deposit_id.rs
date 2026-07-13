use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `Deposit` aggregate.
#[aggregate_id]
pub struct DepositId(Uuid);

impl DepositId {
    /// Creates a new deposit ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for DepositId {
    fn default() -> Self {
        Self::new()
    }
}
