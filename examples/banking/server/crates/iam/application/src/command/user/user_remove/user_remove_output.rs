use banking_iam_domain::user::{UserRemoveResult, UserStatusRejectionReason};
use serde::{Deserialize, Serialize};

/// Returned after a user removal request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserRemoveOutput {
    Removed,
    Rejected { reason: UserStatusRejectionReason },
}

impl From<UserRemoveResult> for UserRemoveOutput {
    fn from(value: UserRemoveResult) -> Self {
        match value {
            UserRemoveResult::Removed => Self::Removed,
            UserRemoveResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
