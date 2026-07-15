use banking_ledger_domain::withdrawal::WithdrawalCompleteRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a withdrawal completion request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalCompleteOutput {
    Completed,
    Rejected {
        reason: WithdrawalCompleteRejectionReason,
    },
}
