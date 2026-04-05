use banking_iam_domain::OrganizationMembershipId;
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization membership.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMembershipCreateOutput {
    pub organization_membership_id: OrganizationMembershipId,
}

impl OrganizationMembershipCreateOutput {
    /// Creates a new organization-membership-create output.
    pub fn new(organization_membership_id: OrganizationMembershipId) -> Self {
        Self {
            organization_membership_id,
        }
    }
}
