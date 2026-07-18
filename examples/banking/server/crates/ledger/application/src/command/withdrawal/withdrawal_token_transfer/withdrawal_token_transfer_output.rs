use banking_ledger_domain::withdrawal::WithdrawalTokenTransferRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an external withdrawal token transfer attempt is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalTokenTransferOutput {
    TokenTransferred,
    Rejected {
        reason: WithdrawalTokenTransferRejectionReason,
    },
}
