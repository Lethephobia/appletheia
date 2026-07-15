use appletheia::command;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

/// Completes the specified withdrawal.
#[command(name = "withdrawal_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalCompleteCommand {
    pub withdrawal_id: WithdrawalId,
}
