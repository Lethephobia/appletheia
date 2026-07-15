use super::CurrencyDescriptionChangeRejectionReason;

/// Describes the domain outcome of a currency description change request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyDescriptionChangeResult {
    Changed,
    Rejected {
        reason: CurrencyDescriptionChangeRejectionReason,
    },
}
