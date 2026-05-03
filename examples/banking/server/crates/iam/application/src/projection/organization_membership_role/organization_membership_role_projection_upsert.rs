use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

/// Attributes required to upsert a normalized organization membership role projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipRoleProjectionUpsert {
    pub organization_membership_id: OrganizationMembershipId,
    pub role: OrganizationRole,
}
