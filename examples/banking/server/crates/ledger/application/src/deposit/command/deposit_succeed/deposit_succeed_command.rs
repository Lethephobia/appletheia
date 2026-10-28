use appletheia::command;
use banking_ledger_domain::deposit::DepositId;
use serde::{Deserialize, Serialize};

/// Records deposit success after internal accounting has been applied.
#[command(name = "deposit_succeed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositSucceedCommand {
    pub deposit_id: DepositId,
}
