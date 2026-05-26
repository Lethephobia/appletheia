use super::WithdrawalTokenTransferRejectionReason;

/// Describes the domain outcome of recording an external token transfer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalTokenTransferResult {
    TokenTransferred,
    Rejected {
        reason: WithdrawalTokenTransferRejectionReason,
    },
}
