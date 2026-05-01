use serde::{Deserialize, Serialize};

/// Describes why a transfer cancel request was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TransferCancelRejectionReason {
    AlreadyCompleted,
    AlreadyFailed,
    AlreadyCancelled,
    AlreadyRejected,
}
