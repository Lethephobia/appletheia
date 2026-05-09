use banking_iam_domain::user::{UserDisplayNameChangeRejectionReason, UserDisplayNameChangeResult};
use serde::{Deserialize, Serialize};

/// Returned after a user display name change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserDisplayNameChangeOutput {
    Changed,
    Rejected {
        reason: UserDisplayNameChangeRejectionReason,
    },
}

impl From<UserDisplayNameChangeResult> for UserDisplayNameChangeOutput {
    fn from(value: UserDisplayNameChangeResult) -> Self {
        match value {
            UserDisplayNameChangeResult::Changed => Self::Changed,
            UserDisplayNameChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
