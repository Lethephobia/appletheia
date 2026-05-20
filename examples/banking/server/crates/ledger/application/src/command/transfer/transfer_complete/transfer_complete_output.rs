use banking_ledger_domain::transfer::TransferCompleteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after completing a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferCompleteOutput {
    Completed,
    Rejected {
        reason: TransferCompleteRejectionReason,
    },
}
