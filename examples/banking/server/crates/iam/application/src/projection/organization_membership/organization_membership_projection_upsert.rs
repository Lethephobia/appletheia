use banking_iam_domain::{
    OrganizationId, OrganizationMembershipId, OrganizationMembershipStatus, UserId,
};

/// Attributes required to upsert a normalized organization membership projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipProjectionUpsert {
    pub id: OrganizationMembershipId,
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub status: OrganizationMembershipStatus,
}
