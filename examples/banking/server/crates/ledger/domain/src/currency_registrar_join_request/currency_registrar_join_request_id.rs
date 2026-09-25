use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `CurrencyRegistrarJoinRequest` aggregate.
#[aggregate_id]
pub struct CurrencyRegistrarJoinRequestId(Uuid);

impl CurrencyRegistrarJoinRequestId {
    /// Creates a new currency registrar join request ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyRegistrarJoinRequestId {
    fn default() -> Self {
        Self::new()
    }
}
