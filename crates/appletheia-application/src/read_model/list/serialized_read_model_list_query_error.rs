use thiserror::Error;

/// Reports an invalid or incompatible serialized list query.
#[derive(Debug, Error)]
pub enum SerializedReadModelListQueryError {
    #[error("read model list query must not be null")]
    NullQuery,
    #[error("read model list query serialization failed: {0}")]
    Json(#[from] serde_json::Error),
}
