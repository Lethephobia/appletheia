use serde::{Deserialize, Serialize};

/// Describes why recording a verified deposit settlement was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositSettlementVerifyRejectionReason {
    TokenBindingUnavailable,
    ChainMismatch,
    AlreadyVerified,
    AlreadyCompleted,
    AlreadyFailed,
    AlreadyRejected,
}
