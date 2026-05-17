use thiserror::Error;

/// Describes why a mint metadata name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintMetadataNameError {
    #[error("mint metadata name cannot be empty")]
    Empty,
}
