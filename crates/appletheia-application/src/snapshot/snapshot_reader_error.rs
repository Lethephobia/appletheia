use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotReaderError {
    #[error("snapshot mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("snapshot persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("transaction is not active")]
    NotInTransaction,
}
