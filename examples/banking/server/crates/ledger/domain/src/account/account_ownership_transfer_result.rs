use super::AccountOwnershipTransferRejectionReason;

/// Describes the domain outcome of an ownership transfer request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountOwnershipTransferResult {
    Transferred,
    Rejected {
        reason: AccountOwnershipTransferRejectionReason,
    },
}
