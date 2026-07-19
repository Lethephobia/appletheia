use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};

/// Describes an organization member list membership upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberListMembershipUpsert {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub roles: OrganizationRoles,
}
