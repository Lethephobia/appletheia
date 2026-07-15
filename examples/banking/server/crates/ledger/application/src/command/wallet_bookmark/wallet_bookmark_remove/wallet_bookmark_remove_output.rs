use banking_ledger_domain::wallet_bookmark::WalletBookmarkRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a wallet bookmark removal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WalletBookmarkRemoveOutput {
    Removed,
    Rejected {
        reason: WalletBookmarkRemoveRejectionReason,
    },
}
