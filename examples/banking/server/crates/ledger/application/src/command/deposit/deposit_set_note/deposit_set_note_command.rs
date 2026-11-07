use appletheia::command;
use banking_ledger_domain::deposit::{DepositId, DepositNote};
use serde::{Deserialize, Serialize};

#[command(name = "deposit_set_note")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSetNoteCommand {
    pub deposit_id: DepositId,
    pub note: Option<DepositNote>,
}
