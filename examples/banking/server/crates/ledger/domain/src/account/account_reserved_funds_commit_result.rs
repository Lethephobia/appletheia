use super::AccountReservedFundsCommitRejectionReason;

/// Describes the domain outcome of a reserved funds commit request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountReservedFundsCommitResult {
    Committed,
    Rejected {
        reason: AccountReservedFundsCommitRejectionReason,
    },
}
