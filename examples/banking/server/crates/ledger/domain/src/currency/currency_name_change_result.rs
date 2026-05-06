use super::CurrencyNameChangeRejectionReason;

/// Describes the domain outcome of a currency name change request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyNameChangeResult {
    Changed,
    Rejected {
        reason: CurrencyNameChangeRejectionReason,
    },
}
