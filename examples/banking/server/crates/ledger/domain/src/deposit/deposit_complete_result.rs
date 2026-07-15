use super::DepositCompleteRejectionReason;

/// Describes the domain outcome of completing a deposit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DepositCompleteResult {
    Completed,
    Rejected {
        reason: DepositCompleteRejectionReason,
    },
}
