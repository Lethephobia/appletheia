use banking_iam_domain::user::UserStatusRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after a user deactivation request is applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserDeactivateOutput {
    Deactivated,
    Rejected { reason: UserStatusRejectionReason },
}
