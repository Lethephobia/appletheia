use banking_iam_domain::OrganizationMembershipRemoveRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRemoveOutput {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}
