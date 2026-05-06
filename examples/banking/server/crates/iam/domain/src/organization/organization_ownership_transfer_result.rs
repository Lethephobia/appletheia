use super::OrganizationOwnershipTransferRejectionReason;

/// Describes the domain outcome of an organization operation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationOwnershipTransferResult {
    Transferred,
    Rejected {
        reason: OrganizationOwnershipTransferRejectionReason,
    },
}
