use serde::{Deserialize, Serialize};

/// Describes why a user picture change operation was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureChangeRejectionReason {
    Inactive,
    Removed,
}
