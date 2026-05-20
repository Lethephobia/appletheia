use banking_iam_domain::user::UserBioChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a user bio change request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserBioChangeOutput {
    Changed,
    Rejected {
        reason: UserBioChangeRejectionReason,
    },
}
