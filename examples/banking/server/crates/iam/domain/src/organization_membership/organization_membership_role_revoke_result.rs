use super::OrganizationMembershipRoleRevokeRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRoleRevokeResult {
    Revoked,
    Rejected {
        reason: OrganizationMembershipRoleRevokeRejectionReason,
    },
}
