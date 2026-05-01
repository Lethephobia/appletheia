use super::OrganizationMembershipRoleGrantRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRoleGrantResult {
    Granted,
    Rejected {
        reason: OrganizationMembershipRoleGrantRejectionReason,
    },
}
