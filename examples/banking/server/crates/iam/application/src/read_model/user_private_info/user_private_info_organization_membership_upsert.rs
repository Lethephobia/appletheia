use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoOrganizationMembershipUpsert {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
}
