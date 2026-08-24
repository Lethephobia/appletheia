use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies an `CurrencyRegistrarInvitation` aggregate.
#[aggregate_id]
pub struct CurrencyRegistrarInvitationId(Uuid);

impl CurrencyRegistrarInvitationId {
    /// Creates a new currency registrar invitation ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CurrencyRegistrarInvitationId {
    fn default() -> Self {
        Self::new()
    }
}
