use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// The output returned after requesting a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequestOutput {
    pub transfer_id: TransferId,
}

impl TransferRequestOutput {
    /// Creates a new transfer-request output.
    pub fn new(transfer_id: TransferId) -> Self {
        Self { transfer_id }
    }
}
