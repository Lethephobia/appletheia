use thiserror::Error;

/// Represents errors returned while publishing mint metadata.
#[derive(Debug, Error)]
pub enum MintMetadataPublisherError {
    #[error("mint metadata publisher backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
