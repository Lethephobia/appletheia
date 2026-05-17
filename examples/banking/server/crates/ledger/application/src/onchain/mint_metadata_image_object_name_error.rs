use thiserror::Error;

/// Describes why a mint metadata image object name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintMetadataImageObjectNameError {
    #[error("mint metadata image object name cannot be empty")]
    Empty,
}
