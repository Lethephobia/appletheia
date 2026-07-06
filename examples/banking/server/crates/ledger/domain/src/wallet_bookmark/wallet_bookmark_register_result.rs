use super::WalletBookmarkId;

/// Describes the domain outcome of a wallet bookmark registration.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WalletBookmarkRegisterResult {
    Registered {
        wallet_bookmark_id: WalletBookmarkId,
    },
}
