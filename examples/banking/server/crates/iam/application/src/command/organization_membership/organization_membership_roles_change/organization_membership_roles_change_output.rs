use banking_iam_domain::{
    OrganizationMembershipRolesChangeRejectionReason, OrganizationMembershipRolesChangeResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipRolesChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
}

impl From<OrganizationMembershipRolesChangeResult> for OrganizationMembershipRolesChangeOutput {
    fn from(value: OrganizationMembershipRolesChangeResult) -> Self {
        match value {
            OrganizationMembershipRolesChangeResult::Changed => Self::Changed,
            OrganizationMembershipRolesChangeResult::Rejected { reason } => {
                Self::Rejected { reason }
            }
        }
    }
}
