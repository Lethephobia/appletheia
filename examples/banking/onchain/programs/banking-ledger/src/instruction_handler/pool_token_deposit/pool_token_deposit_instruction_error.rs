use anchor_lang::prelude::*;

#[error_code]
pub enum PoolTokenDepositInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint state is not initialized")]
    MintStateNotInitialized,
    #[msg("mint state conflicts with the expected layout")]
    MintStateConflict,
    #[msg("mint account conflicts with mint state")]
    MintAccountConflict,
    #[msg("pool token deposit receipt conflicts with this deposit")]
    PoolTokenDepositReceiptConflict,
}
