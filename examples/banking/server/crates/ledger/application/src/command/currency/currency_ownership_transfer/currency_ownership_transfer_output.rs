use banking_ledger_domain::currency::CurrencyOwnershipTransferRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after transferring currency ownership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyOwnershipTransferOutput {
    Transferred,
    Rejected {
        reason: CurrencyOwnershipTransferRejectionReason,
    },
}
