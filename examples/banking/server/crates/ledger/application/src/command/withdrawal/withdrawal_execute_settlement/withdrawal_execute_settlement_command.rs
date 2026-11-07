use appletheia::command;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

/// Executes the external transfer step for a withdrawal.
#[command(name = "withdrawal_execute_settlement")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalExecuteSettlementCommand {
    pub withdrawal_id: WithdrawalId,
}
