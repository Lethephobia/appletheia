use banking_ledger_domain::currency::CurrencyMintAccountCreationRequestRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after requesting currency mint account creation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyMintAccountRequestOutput {
    Requested,
    Rejected {
        reason: CurrencyMintAccountCreationRequestRejectionReason,
    },
}
