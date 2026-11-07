use appletheia::command;
use banking_ledger_domain::withdrawal::{WithdrawalId, WithdrawalNote};
use serde::{Deserialize, Serialize};

#[command(name = "withdrawal_set_note")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalSetNoteCommand {
    pub withdrawal_id: WithdrawalId,
    pub note: Option<WithdrawalNote>,
}
