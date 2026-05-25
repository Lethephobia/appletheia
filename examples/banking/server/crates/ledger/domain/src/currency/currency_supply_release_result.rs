use super::CurrencySupplyReleaseRejectionReason;

/// Describes the domain outcome of a release-supply request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySupplyReleaseResult {
    Released,
    Rejected {
        reason: CurrencySupplyReleaseRejectionReason,
    },
}
