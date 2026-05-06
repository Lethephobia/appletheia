use super::AccountReservedFundsReleaseRejectionReason;

/// Describes the domain outcome of a reserved funds release request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountReservedFundsReleaseResult {
    Released,
    Rejected {
        reason: AccountReservedFundsReleaseRejectionReason,
    },
}
