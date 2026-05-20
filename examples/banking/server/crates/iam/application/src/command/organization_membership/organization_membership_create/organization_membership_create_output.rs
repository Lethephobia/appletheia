use banking_iam_domain::{OrganizationMembershipCreateRejectionReason, OrganizationMembershipId};
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationMembershipCreateOutput {
    Created {
        organization_membership_id: OrganizationMembershipId,
    },
    Rejected {
        organization_membership_id: OrganizationMembershipId,
        reason: OrganizationMembershipCreateRejectionReason,
    },
}
