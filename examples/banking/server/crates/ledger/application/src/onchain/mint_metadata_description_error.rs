use thiserror::Error;

/// Describes why a mint metadata description is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintMetadataDescriptionError {
    #[error("mint metadata description cannot be empty")]
    Empty,
}
