use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `CurrencyIssuance` aggregate.
#[aggregate_id]
pub struct CurrencyIssuanceId(Uuid);

impl CurrencyIssuanceId {
    /// Creates a new currency issuance ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyIssuanceId {
    fn default() -> Self {
        Self::new()
    }
}
