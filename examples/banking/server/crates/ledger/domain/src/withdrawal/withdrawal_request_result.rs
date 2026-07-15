use super::WithdrawalRequestRejectionReason;

/// Describes the domain outcome of a withdrawal request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalRequestResult {
    Requested,
    Rejected {
        reason: WithdrawalRequestRejectionReason,
    },
}
