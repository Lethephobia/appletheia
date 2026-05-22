use thiserror::Error;

/// Describes why a mint metadata symbol is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintMetadataSymbolError {
    #[error("mint metadata symbol cannot be empty")]
    Empty,
}
