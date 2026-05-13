use super::OrganizationMembershipRolesChangeRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRolesChangeResult {
    Changed,
    Rejected {
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
}
