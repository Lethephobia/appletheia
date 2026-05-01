use banking_ledger_domain::transfer::{
    TransferId, TransferRequestRejectionReason, TransferRequestResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after requesting a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferRequestOutput {
    Requested {
        transfer_id: TransferId,
    },
    Rejected {
        reason: TransferRequestRejectionReason,
    },
}

impl From<TransferRequestResult> for TransferRequestOutput {
    fn from(value: TransferRequestResult) -> Self {
        match value {
            TransferRequestResult::Requested { transfer_id } => Self::Requested { transfer_id },
            TransferRequestResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
