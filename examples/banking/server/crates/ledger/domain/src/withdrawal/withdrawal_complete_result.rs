use super::WithdrawalCompleteRejectionReason;

/// Describes the domain outcome of completing a withdrawal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalCompleteResult {
    Completed,
    Rejected {
        reason: WithdrawalCompleteRejectionReason,
    },
}
