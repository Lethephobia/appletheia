use std::error::Error;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum RelationshipStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("relationship persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("relationship mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("invalid relationship row")]
    InvalidRow,
}
