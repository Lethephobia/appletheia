use banking_iam_domain::user::{UserActivateResult, UserStatusRejectionReason};
use serde::{Deserialize, Serialize};

/// Returned after a user activation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserActivateOutput {
    Activated,
    Rejected { reason: UserStatusRejectionReason },
}

impl From<UserActivateResult> for UserActivateOutput {
    fn from(value: UserActivateResult) -> Self {
        match value {
            UserActivateResult::Activated => Self::Activated,
            UserActivateResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
