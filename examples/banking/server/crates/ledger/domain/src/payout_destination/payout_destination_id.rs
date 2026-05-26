use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `PayoutDestination` aggregate.
#[aggregate_id]
pub struct PayoutDestinationId(Uuid);

impl PayoutDestinationId {
    /// Creates a new payout destination ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for PayoutDestinationId {
    fn default() -> Self {
        Self::new()
    }
}
