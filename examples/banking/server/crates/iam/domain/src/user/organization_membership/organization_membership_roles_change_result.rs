use super::OrganizationMembershipRolesChangeRejectionReason;

/// Describes the domain outcome of changing user organization membership roles.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRolesChangeResult {
    Changed,
    Rejected {
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
}
