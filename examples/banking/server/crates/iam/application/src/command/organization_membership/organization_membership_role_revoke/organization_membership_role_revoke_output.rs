use banking_iam_domain::{
    OrganizationMembershipRoleRevokeRejectionReason, OrganizationMembershipRoleRevokeResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRoleRevokeOutput {
    Revoked,
    Rejected {
        reason: OrganizationMembershipRoleRevokeRejectionReason,
    },
}

impl From<OrganizationMembershipRoleRevokeResult> for OrganizationMembershipRoleRevokeOutput {
    fn from(value: OrganizationMembershipRoleRevokeResult) -> Self {
        match value {
            OrganizationMembershipRoleRevokeResult::Revoked => Self::Revoked,
            OrganizationMembershipRoleRevokeResult::Rejected { reason } => {
                Self::Rejected { reason }
            }
        }
    }
}
