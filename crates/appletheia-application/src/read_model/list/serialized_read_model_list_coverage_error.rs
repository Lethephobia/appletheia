use thiserror::Error;

/// Reports an invalid or incompatible serialized list coverage.
#[derive(Debug, Error)]
pub enum SerializedReadModelListCoverageError {
    #[error("read model list coverage must not be null")]
    NullCoverage,
    #[error("read model list coverage serialization failed: {0}")]
    Json(#[from] serde_json::Error),
}
