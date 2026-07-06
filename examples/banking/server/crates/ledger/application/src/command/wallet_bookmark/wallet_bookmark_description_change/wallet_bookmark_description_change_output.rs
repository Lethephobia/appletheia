use banking_ledger_domain::wallet_bookmark::WalletBookmarkDescriptionChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after changing a wallet bookmark description.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WalletBookmarkDescriptionChangeOutput {
    Changed,
    Rejected {
        reason: WalletBookmarkDescriptionChangeRejectionReason,
    },
}
