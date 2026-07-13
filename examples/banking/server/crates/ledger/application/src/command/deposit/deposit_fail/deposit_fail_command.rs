use appletheia::command;
use banking_ledger_domain::deposit::{DepositFailureReason, DepositId};
use serde::{Deserialize, Serialize};

/// Fails a deposit workflow.
#[command(name = "deposit_fail")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositFailCommand {
    pub deposit_id: DepositId,
    pub reason: DepositFailureReason,
}
