use super::CurrencyRemoveRejectionReason;

/// Describes the domain outcome of a currency remove request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyRemoveResult {
    Removed,
    Rejected {
        reason: CurrencyRemoveRejectionReason,
    },
}
