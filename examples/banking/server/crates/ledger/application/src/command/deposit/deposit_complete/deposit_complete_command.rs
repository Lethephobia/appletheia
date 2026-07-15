use appletheia::command;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

/// Completes a deposit workflow.
#[command(name = "deposit_complete")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositCompleteCommand {
    pub deposit_id: DepositId,
}
