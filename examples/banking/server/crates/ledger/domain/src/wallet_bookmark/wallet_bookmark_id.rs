use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::Uuid;

/// Identifies a `WalletBookmark` aggregate.
#[aggregate_id]
pub struct WalletBookmarkId(Uuid);

impl WalletBookmarkId {
    /// Creates a new wallet bookmark ID.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for WalletBookmarkId {
    fn default() -> Self {
        Self::new()
    }
}
