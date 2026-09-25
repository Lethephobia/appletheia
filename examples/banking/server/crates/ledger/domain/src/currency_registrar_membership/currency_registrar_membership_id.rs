use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a CurrencyRegistrarMembership aggregate.
#[aggregate_id]
pub struct CurrencyRegistrarMembershipId(Uuid);

impl CurrencyRegistrarMembershipId {
    /// Creates a new CurrencyRegistrarMembership ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyRegistrarMembershipId {
    fn default() -> Self {
        Self::new()
    }
}
