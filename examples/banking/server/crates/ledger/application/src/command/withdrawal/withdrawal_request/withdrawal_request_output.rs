use banking_ledger_domain::withdrawal::{WithdrawalId, WithdrawalRequestRejectionReason};
use serde::{Deserialize, Serialize};

/// Returned after a withdrawal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalRequestOutput {
    Requested {
        withdrawal_id: WithdrawalId,
    },
    Rejected {
        withdrawal_id: WithdrawalId,
        reason: WithdrawalRequestRejectionReason,
    },
}
