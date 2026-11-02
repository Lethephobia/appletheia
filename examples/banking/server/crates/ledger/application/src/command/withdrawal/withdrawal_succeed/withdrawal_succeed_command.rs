use appletheia::command;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

/// Records withdrawal success after internal accounting has been committed.
#[command(name = "withdrawal_succeed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalSucceedCommand {
    pub withdrawal_id: WithdrawalId,
}
