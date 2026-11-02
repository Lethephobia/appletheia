use appletheia::command;
use banking_ledger_domain::withdrawal::{WithdrawalFailureReason, WithdrawalId};
use serde::{Deserialize, Serialize};

/// Fails the specified withdrawal.
#[command(name = "withdrawal_fail")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalFailCommand {
    pub withdrawal_id: WithdrawalId,
    pub reason: WithdrawalFailureReason,
}
