use anchor_lang::prelude::*;

#[error_code]
pub enum WithdrawalSettleInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("withdrawal settlement marker conflicts with this withdrawal")]
    WithdrawalSettlementReceiptConflict,
}
