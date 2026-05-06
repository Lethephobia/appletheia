use super::CurrencySupplyIncreaseRejectionReason;

/// Describes the domain outcome of an increase-supply request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySupplyIncreaseResult {
    Increased,
    Rejected {
        reason: CurrencySupplyIncreaseRejectionReason,
    },
}
