use super::CurrencyActivateRejectionReason;

/// Describes the domain outcome of a currency activation request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyActivateResult {
    Activated,
    Rejected {
        reason: CurrencyActivateRejectionReason,
    },
}
