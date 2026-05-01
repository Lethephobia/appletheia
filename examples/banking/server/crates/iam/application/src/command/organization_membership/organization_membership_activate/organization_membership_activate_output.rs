use banking_iam_domain::{
    OrganizationMembershipActivateRejectionReason, OrganizationMembershipActivateResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization membership operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipActivateOutput {
    Activated,
    Rejected {
        reason: OrganizationMembershipActivateRejectionReason,
    },
}

impl From<OrganizationMembershipActivateResult> for OrganizationMembershipActivateOutput {
    fn from(value: OrganizationMembershipActivateResult) -> Self {
        match value {
            OrganizationMembershipActivateResult::Activated => Self::Activated,
            OrganizationMembershipActivateResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
