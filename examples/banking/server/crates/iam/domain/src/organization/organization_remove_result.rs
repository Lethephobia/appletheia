use super::OrganizationRemoveRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationRemoveResult {
    Removed,
    Rejected {
        reason: OrganizationRemoveRejectionReason,
    },
}
