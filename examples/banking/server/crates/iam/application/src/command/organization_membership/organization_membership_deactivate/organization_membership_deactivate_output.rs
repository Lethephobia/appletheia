use banking_iam_domain::{
    OrganizationMembershipDeactivateRejectionReason, OrganizationMembershipDeactivateResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipDeactivateOutput {
    Deactivated,
    Rejected {
        reason: OrganizationMembershipDeactivateRejectionReason,
    },
}

impl From<OrganizationMembershipDeactivateResult> for OrganizationMembershipDeactivateOutput {
    fn from(value: OrganizationMembershipDeactivateResult) -> Self {
        match value {
            OrganizationMembershipDeactivateResult::Deactivated => Self::Deactivated,
            OrganizationMembershipDeactivateResult::Rejected { reason } => {
                Self::Rejected { reason }
            }
        }
    }
}
