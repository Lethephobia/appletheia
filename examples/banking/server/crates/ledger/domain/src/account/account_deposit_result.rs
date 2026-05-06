use super::AccountDepositRejectionReason;

/// Describes the domain outcome of a deposit request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccountDepositResult {
    Deposited,
    Rejected {
        reason: AccountDepositRejectionReason,
    },
}
