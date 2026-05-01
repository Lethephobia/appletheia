use banking_ledger_domain::transfer::{TransferCompleteRejectionReason, TransferCompleteResult};
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

impl From<TransferCompleteResult> for TransferCompleteOutput {
    fn from(value: TransferCompleteResult) -> Self {
        match value {
            TransferCompleteResult::Completed => Self::Completed,
            TransferCompleteResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
