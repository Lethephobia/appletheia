use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// The output returned after initiating a transfer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferInitiateOutput {
    pub transfer_id: TransferId,
}

impl TransferInitiateOutput {
    /// Creates a new transfer-initiate output.
    pub fn new(transfer_id: TransferId) -> Self {
        Self { transfer_id }
    }
}
