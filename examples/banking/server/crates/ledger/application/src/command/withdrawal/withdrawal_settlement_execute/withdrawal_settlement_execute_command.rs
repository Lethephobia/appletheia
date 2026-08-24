use appletheia::command;
use banking_ledger_domain::withdrawal::WithdrawalId;
use serde::{Deserialize, Serialize};

/// Executes the external transfer step for a withdrawal.
#[command(name = "withdrawal_settlement_execute")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalSettlementExecuteCommand {
    pub withdrawal_id: WithdrawalId,
}
