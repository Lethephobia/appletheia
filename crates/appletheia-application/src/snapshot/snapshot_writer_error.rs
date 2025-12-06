use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotWriterError {
    #[error("snapshot persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("transaction is not active")]
    NotInTransaction,

    #[error("json serialization error: {0}")]
    Json(#[source] serde_json::Error),
}
