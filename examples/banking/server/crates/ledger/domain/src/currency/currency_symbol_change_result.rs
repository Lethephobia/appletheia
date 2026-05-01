use super::CurrencySymbolChangeRejectionReason;

/// Describes the domain outcome of a currency symbol change request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySymbolChangeResult {
    Changed,
    Rejected {
        reason: CurrencySymbolChangeRejectionReason,
    },
}
