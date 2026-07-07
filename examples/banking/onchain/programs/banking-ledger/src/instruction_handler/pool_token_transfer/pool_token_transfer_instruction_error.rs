use anchor_lang::prelude::*;

#[error_code]
pub enum PoolTokenTransferInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint state is not initialized")]
    MintStateNotInitialized,
    #[msg("mint state conflicts with the requested pool token transfer")]
    MintStateConflict,
    #[msg("mint account conflicts with the requested pool token transfer")]
    MintAccountConflict,
    #[msg("pool token transfer marker conflicts with the requested transfer")]
    PoolTokenTransferMarkerConflict,
}
