use banking_iam_domain::user::{UserProfileChangeRejectionReason, UserProfileChangeResult};
use serde::{Deserialize, Serialize};

/// Returned after a user picture change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserPictureChangeOutput {
    Changed,
    Rejected {
        reason: UserProfileChangeRejectionReason,
    },
}

impl From<UserProfileChangeResult> for UserPictureChangeOutput {
    fn from(value: UserProfileChangeResult) -> Self {
        match value {
            UserProfileChangeResult::Changed => Self::Changed,
            UserProfileChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
