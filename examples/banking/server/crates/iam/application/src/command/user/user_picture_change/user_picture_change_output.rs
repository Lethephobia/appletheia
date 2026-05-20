use banking_iam_domain::user::UserPictureChangeRejectionReason;
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
