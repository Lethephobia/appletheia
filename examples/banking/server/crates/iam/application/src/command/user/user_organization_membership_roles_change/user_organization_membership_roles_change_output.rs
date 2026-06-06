use banking_iam_domain::OrganizationMembershipRolesChangeRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after changing a user's roles in an organization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserOrganizationMembershipRolesChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
}
