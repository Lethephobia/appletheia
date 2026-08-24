use anchor_lang::prelude::*;

#[error_code]
pub enum DepositSettleInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("deposit settlement receipt conflicts with this deposit")]
    DepositSettlementReceiptConflict,
}
