use appletheia::command;
use banking_ledger_domain::transfer::{TransferId, TransferNote};
use serde::{Deserialize, Serialize};

#[command(name = "transfer_set_note")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSetNoteCommand {
    pub transfer_id: TransferId,
    pub note: Option<TransferNote>,
}
