use appletheia::command;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Fails the specified transfer.
#[command(name = "transfer_fail")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferFailCommand {
    pub transfer_id: TransferId,
}
