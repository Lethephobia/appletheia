use super::OrganizationMembershipRemoveRejectionReason;

/// Describes the domain outcome of an organization membership operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationMembershipRemoveResult {
    Removed,
    Rejected {
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}
