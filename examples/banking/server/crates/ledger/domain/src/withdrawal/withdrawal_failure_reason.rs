use serde::{Deserialize, Serialize};

/// Describes why a withdrawal failed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalFailureReason {
    FundsReserveRejected,
    SettlementExecuteRejected,
    ReservedFundsReleaseRejected,
    ReservedFundsCommitRejected,
}
