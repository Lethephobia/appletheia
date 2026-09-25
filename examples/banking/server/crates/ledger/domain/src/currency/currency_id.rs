use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a Currency aggregate.
#[aggregate_id]
pub struct CurrencyId(Uuid);

impl CurrencyId {
    /// Creates a new Currency ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyId {
    fn default() -> Self {
        Self::new()
    }
}
