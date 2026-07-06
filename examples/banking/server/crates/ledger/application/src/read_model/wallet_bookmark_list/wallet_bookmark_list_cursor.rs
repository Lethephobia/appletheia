use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkId;

/// Cursor for wallet bookmark list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WalletBookmarkListCursor {
    pub created_at: EventOccurredAt,
    pub wallet_bookmark_id: WalletBookmarkId,
}
