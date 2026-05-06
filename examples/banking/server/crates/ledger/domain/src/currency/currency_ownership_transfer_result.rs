use super::CurrencyOwnershipTransferRejectionReason;

/// Describes the domain outcome of a currency ownership transfer request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyOwnershipTransferResult {
    Transferred,
    Rejected {
        reason: CurrencyOwnershipTransferRejectionReason,
    },
}
