use super::WalletBookmarkDescriptionChangeRejectionReason;

/// Describes the domain outcome of changing a wallet bookmark description.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WalletBookmarkDescriptionChangeResult {
    Changed,
    Rejected {
        reason: WalletBookmarkDescriptionChangeRejectionReason,
    },
}
