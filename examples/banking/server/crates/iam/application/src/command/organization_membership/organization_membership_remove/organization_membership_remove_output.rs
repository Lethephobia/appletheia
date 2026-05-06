use banking_iam_domain::{
    OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRemoveResult,
};
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

impl From<OrganizationMembershipRemoveResult> for OrganizationMembershipRemoveOutput {
    fn from(value: OrganizationMembershipRemoveResult) -> Self {
        match value {
            OrganizationMembershipRemoveResult::Removed => Self::Removed,
            OrganizationMembershipRemoveResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
