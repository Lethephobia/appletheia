use anchor_lang::prelude::*;

#[error_code]
pub enum PoolTokenAccountEnsureInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint state is not initialized")]
    MintStateNotInitialized,
    #[msg("mint state conflicts with the requested pool token account ensure")]
    MintStateConflict,
}
