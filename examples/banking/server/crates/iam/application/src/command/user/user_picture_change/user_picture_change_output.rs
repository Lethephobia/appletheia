use banking_iam_domain::user::{UserPictureChangeRejectionReason, UserPictureChangeResult};
use serde::{Deserialize, Serialize};

/// Returned after a user picture change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureChangeOutput {
    Changed,
    Rejected {
        reason: UserPictureChangeRejectionReason,
    },
}

impl From<UserPictureChangeResult> for UserPictureChangeOutput {
    fn from(value: UserPictureChangeResult) -> Self {
        match value {
            UserPictureChangeResult::Changed => Self::Changed,
            UserPictureChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
