use serde::{Deserialize, Serialize};

/// Describes why recording a deposit token transfer was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DepositTokenTransferRecordRejectionReason {
    AlreadyTokenTransferred,
    AlreadyCompleted,
    AlreadyFailed,
    AlreadyRejected,
}
