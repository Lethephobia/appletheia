use serde::{Deserialize, Serialize};

/// Describes why recording a withdrawal token transfer was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum WithdrawalTokenTransferRejectionReason {
    AlreadyTokenTransferred,
    AlreadyCompleted,
    AlreadyFailed,
    AlreadyRejected,
}
