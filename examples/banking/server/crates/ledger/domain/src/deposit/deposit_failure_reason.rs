use serde::{Deserialize, Serialize};

/// Describes why a deposit failed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositFailureReason {
    AccountDepositRejected,
}
