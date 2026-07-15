use super::CurrencySupplyReserveRejectionReason;

/// Describes the domain outcome of a reserve-supply request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CurrencySupplyReserveResult {
    Reserved,
    Rejected {
        reason: CurrencySupplyReserveRejectionReason,
    },
}
