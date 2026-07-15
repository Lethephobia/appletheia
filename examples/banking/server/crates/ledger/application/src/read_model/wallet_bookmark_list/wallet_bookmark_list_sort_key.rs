/// Sort key for wallet bookmark list reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WalletBookmarkListSortKey {
    CreatedAt,
    WalletBookmarkId,
}
