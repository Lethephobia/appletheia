use anchor_lang::prelude::*;

#[error_code]
pub enum MintSupplySyncInstructionError {
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint state is not initialized")]
    MintStateNotInitialized,
    #[msg("mint state conflicts with the requested mint supply sync")]
    MintStateConflict,
    #[msg("mint account conflicts with the requested mint supply sync")]
    MintAccountConflict,
}
