use super::CurrencyDeactivateRejectionReason;

/// Describes the domain outcome of a currency deactivation request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencyDeactivateResult {
    Deactivated,
    Rejected {
        reason: CurrencyDeactivateRejectionReason,
    },
}
