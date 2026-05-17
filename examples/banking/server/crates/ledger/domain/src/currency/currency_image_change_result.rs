use super::CurrencyImageChangeRejectionReason;

/// Describes the domain outcome of a currency image change request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyImageChangeResult {
    Changed,
    Rejected {
        reason: CurrencyImageChangeRejectionReason,
    },
}
