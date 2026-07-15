use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `Withdrawal` aggregate.
#[aggregate_id]
pub struct WithdrawalId(Uuid);

impl WithdrawalId {
    /// Creates a new withdrawal id.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for WithdrawalId {
    fn default() -> Self {
        Self::new()
    }
}
