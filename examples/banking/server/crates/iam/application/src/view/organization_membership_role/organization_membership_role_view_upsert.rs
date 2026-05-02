use banking_iam_domain::{OrganizationMembershipId, OrganizationRole};

/// Attributes required to upsert a normalized organization membership role view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipRoleViewUpsert {
    pub organization_membership_id: OrganizationMembershipId,
    pub role: OrganizationRole,
}
