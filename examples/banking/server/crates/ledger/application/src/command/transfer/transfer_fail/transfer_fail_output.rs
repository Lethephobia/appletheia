use banking_ledger_domain::transfer::{TransferFailRejectionReason, TransferFailResult};
use serde::{Deserialize, Serialize};

/// Returned after failing a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferFailOutput {
    Failed,
    Rejected { reason: TransferFailRejectionReason },
}

impl From<TransferFailResult> for TransferFailOutput {
    fn from(value: TransferFailResult) -> Self {
        match value {
            TransferFailResult::Failed => Self::Failed,
            TransferFailResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
