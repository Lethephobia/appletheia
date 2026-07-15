use super::WalletBookmarkRemoveRejectionReason;

/// Describes the domain outcome of removing a wallet bookmark.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WalletBookmarkRemoveResult {
    Removed,
    Rejected {
        reason: WalletBookmarkRemoveRejectionReason,
    },
}
