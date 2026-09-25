use serde::{Deserialize, Serialize};

/// Represents the lifecycle status of a withdrawal.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalStatus {
    Pending,
    SettlementExecuted,
    Completed,
    Failed,
    Rejected,
}
