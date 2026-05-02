use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

/// Represents a normalized organization membership role view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipRoleView {
    pub organization_membership_id: OrganizationMembershipId,
    pub role: OrganizationRole,
}
