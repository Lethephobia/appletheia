use serde::{Deserialize, Serialize};

/// Describes why a user identity email operation was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserIdentityEmailChangeRejectionReason {
    Inactive,
    Removed,
    NotFound,
}
