use super::WithdrawalFailRejectionReason;

/// Describes the domain outcome of failing a withdrawal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalFailResult {
    Failed,
    Rejected {
        reason: WithdrawalFailRejectionReason,
    },
}
