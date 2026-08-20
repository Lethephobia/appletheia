use super::OrganizationMembershipRemoveRejectionReason;

/// Describes the domain outcome of removing an organization membership.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRemoveResult {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}
