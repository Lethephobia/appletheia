use super::{WithdrawalId, WithdrawalRequestRejectionReason};

/// Describes the domain outcome of a withdrawal request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalRequestResult {
    Requested {
        withdrawal_id: WithdrawalId,
    },
    Rejected {
        withdrawal_id: WithdrawalId,
        reason: WithdrawalRequestRejectionReason,
    },
}
