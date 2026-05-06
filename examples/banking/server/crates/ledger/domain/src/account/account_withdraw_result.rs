use super::AccountWithdrawRejectionReason;

/// Describes the domain outcome of a withdraw request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountWithdrawResult {
    Withdrawn,
    Rejected {
        reason: AccountWithdrawRejectionReason,
    },
}
