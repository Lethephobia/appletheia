use serde::{Deserialize, Serialize};

/// Describes why recording an executed withdrawal settlement was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalSettlementExecuteRejectionReason {
    TokenBindingUnavailable,
    AlreadyExecuted,
    AlreadyCompleted,
    AlreadyFailed,
    AlreadyRejected,
}
