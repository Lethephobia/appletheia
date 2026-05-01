use super::AccountFundsReserveRejectionReason;

/// Describes the domain outcome of a funds reservation request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountFundsReserveResult {
    Reserved,
    Rejected {
        reason: AccountFundsReserveRejectionReason,
    },
}
