use banking_iam_domain::{OrganizationId, OrganizationMembershipId, OrganizationRoles, UserId};

/// Values used to create or replace an organization membership fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipFragmentUpsert {
    pub organization_membership_id: OrganizationMembershipId,
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
}
