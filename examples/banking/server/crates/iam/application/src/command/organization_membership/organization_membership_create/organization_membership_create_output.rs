use banking_iam_domain::{
    OrganizationMembershipCreateRejectionReason, OrganizationMembershipCreateResult,
    OrganizationMembershipId,
};
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipCreateOutput {
    Created {
        organization_membership_id: OrganizationMembershipId,
    },
    Rejected {
        reason: OrganizationMembershipCreateRejectionReason,
    },
}

impl From<OrganizationMembershipCreateResult> for OrganizationMembershipCreateOutput {
    fn from(value: OrganizationMembershipCreateResult) -> Self {
        match value {
            OrganizationMembershipCreateResult::Created {
                organization_membership_id,
            } => Self::Created {
                organization_membership_id,
            },
            OrganizationMembershipCreateResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
