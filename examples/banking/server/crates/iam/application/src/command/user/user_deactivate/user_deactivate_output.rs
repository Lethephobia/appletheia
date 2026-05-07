use banking_iam_domain::user::{UserDeactivateResult, UserStatusRejectionReason};
use serde::{Deserialize, Serialize};

/// Returned after a user deactivation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserDeactivateOutput {
    Deactivated,
    Rejected { reason: UserStatusRejectionReason },
}

impl From<UserDeactivateResult> for UserDeactivateOutput {
    fn from(value: UserDeactivateResult) -> Self {
        match value {
            UserDeactivateResult::Deactivated => Self::Deactivated,
            UserDeactivateResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
