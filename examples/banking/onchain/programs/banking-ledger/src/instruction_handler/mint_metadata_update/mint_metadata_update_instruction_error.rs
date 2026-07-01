use anchor_lang::prelude::*;

#[error_code]
pub enum MintMetadataUpdateInstructionError {
    #[msg("metadata name is too long")]
    MetadataNameTooLong,
    #[msg("metadata symbol is too long")]
    MetadataSymbolTooLong,
    #[msg("metadata URI is too long")]
    MetadataUriTooLong,
    #[msg("operator is not authorized")]
    UnauthorizedOperator,
    #[msg("mint state is not initialized")]
    MintStateNotInitialized,
    #[msg("mint state conflicts with the requested metadata update")]
    MintStateConflict,
    #[msg("mint metadata account conflicts with the requested metadata update")]
    MintMetadataAccountConflict,
}
