use super::WalletBookmarkDisplayNameChangeRejectionReason;

/// Describes the domain outcome of changing a wallet bookmark display name.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WalletBookmarkDisplayNameChangeResult {
    Changed,
    Rejected {
        reason: WalletBookmarkDisplayNameChangeRejectionReason,
    },
}
