use appletheia::command;
use banking_ledger_domain::transfer::{TransferId, TransferNote};
use serde::{Deserialize, Serialize};

#[command(name = "transfer_note_set")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferNoteSetCommand {
    pub transfer_id: TransferId,
    pub note: Option<TransferNote>,
}
