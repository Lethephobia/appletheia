use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a TokenBinding aggregate.
#[aggregate_id]
pub struct TokenBindingId(Uuid);

impl TokenBindingId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for TokenBindingId {
    fn default() -> Self {
        Self::new()
    }
}
