use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a CurrencyRegistrar aggregate.
#[aggregate_id]
pub struct CurrencyRegistrarId(Uuid);

impl CurrencyRegistrarId {
    /// Creates a new CurrencyRegistrar ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyRegistrarId {
    fn default() -> Self {
        Self::new()
    }
}
