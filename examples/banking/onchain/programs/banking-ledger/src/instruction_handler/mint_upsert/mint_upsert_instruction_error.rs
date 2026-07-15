use anchor_lang::prelude::*;

#[error_code]
pub enum MintUpsertInstructionError {
    #[msg("metadata name is too long")]
    MetadataNameTooLong,
    #[msg("metadata symbol is too long")]
    MetadataSymbolTooLong,
    #[msg("metadata URI is too long")]
    MetadataUriTooLong,
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint account conflicts with the requested mint upsert")]
    MintAccountConflict,
}
