use appletheia::command;
use banking_ledger_domain::transfer::TransferId;
use serde::{Deserialize, Serialize};

/// Records transfer success after the source withdrawal has finished.
#[command(name = "transfer_succeed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSucceedCommand {
    pub transfer_id: TransferId,
}
