use serde::{Deserialize, Serialize};

/// Describes why an owned account closure failed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OwnedAccountClosureFailureReason {
    AccountCloseRejected,
    AccountCloseRecordRejected,
    AccountCloseRejectionRecordRejected,
    PageLoadRejected,
}
