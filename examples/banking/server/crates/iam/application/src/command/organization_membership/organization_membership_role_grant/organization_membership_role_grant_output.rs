use banking_iam_domain::{
    OrganizationMembershipRoleGrantRejectionReason, OrganizationMembershipRoleGrantResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRoleGrantOutput {
    Granted,
    Rejected {
        reason: OrganizationMembershipRoleGrantRejectionReason,
    },
}

impl From<OrganizationMembershipRoleGrantResult> for OrganizationMembershipRoleGrantOutput {
    fn from(value: OrganizationMembershipRoleGrantResult) -> Self {
        match value {
            OrganizationMembershipRoleGrantResult::Granted => Self::Granted,
            OrganizationMembershipRoleGrantResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
