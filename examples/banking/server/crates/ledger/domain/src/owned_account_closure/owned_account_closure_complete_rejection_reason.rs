use serde::{Deserialize, Serialize};

/// Describes why completing an owned account closure was rejected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureCompleteRejectionReason {
    NotInProgress,
    AccountCloseRejected,
    AlreadyCompleted,
    AlreadyFailed,
}
