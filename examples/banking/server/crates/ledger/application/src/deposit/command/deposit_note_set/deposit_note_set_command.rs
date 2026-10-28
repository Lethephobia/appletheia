use appletheia::command;
use banking_ledger_domain::deposit::{DepositId, DepositNote};
use serde::{Deserialize, Serialize};

#[command(name = "deposit_note_set")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositNoteSetCommand {
    pub deposit_id: DepositId,
    pub note: Option<DepositNote>,
}
