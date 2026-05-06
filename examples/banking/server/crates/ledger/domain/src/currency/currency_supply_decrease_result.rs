use super::CurrencySupplyDecreaseRejectionReason;

/// Describes the domain outcome of a decrease-supply request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySupplyDecreaseResult {
    Decreased,
    Rejected {
        reason: CurrencySupplyDecreaseRejectionReason,
    },
}
