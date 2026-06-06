use super::OrganizationMembershipRemoveRejectionReason;

/// Describes the domain outcome of removing a user organization membership.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRemoveResult {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}
