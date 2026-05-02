use banking_iam_domain::{
    OrganizationId, OrganizationMembershipId, OrganizationMembershipStatus, UserId,
};

/// Represents the normalized membership view persisted by read projections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMembershipView {
    pub id: OrganizationMembershipId,
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub status: OrganizationMembershipStatus,
}
