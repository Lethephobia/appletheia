use serde::{Deserialize, Serialize};

/// Describes why a requested transfer failed after orchestration began.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferFailureReason {
    FundsReserveRejected,
    DepositRejected,
    ReservedFundsReleaseRejected,
    ReservedFundsCommitRejected,
}
