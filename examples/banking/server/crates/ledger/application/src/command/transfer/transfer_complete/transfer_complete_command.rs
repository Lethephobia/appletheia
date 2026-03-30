use appletheia::command;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Completes the specified transfer.
#[command(name = "transfer_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferCompleteCommand {
    pub transfer_id: TransferId,
}
