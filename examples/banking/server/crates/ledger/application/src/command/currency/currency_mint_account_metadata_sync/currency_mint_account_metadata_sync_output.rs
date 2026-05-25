use banking_ledger_domain::currency::CurrencyMintAccountMetadataSyncRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a currency mint metadata sync request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyMintAccountMetadataSyncOutput {
    Synced,
    Rejected {
        reason: CurrencyMintAccountMetadataSyncRejectionReason,
    },
}
