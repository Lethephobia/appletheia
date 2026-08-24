use super::WithdrawalSettlementExecuteRejectionReason;

/// Describes the outcome of recording an executed withdrawal settlement.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WithdrawalSettlementExecuteResult {
    Executed,
    Rejected {
        reason: WithdrawalSettlementExecuteRejectionReason,
    },
}
