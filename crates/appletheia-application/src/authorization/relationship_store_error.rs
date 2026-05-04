use std::error::Error as StdError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelationshipStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("relationship persistence error: {0}")]
    Persistence(#[source] Box<dyn StdError + Send + Sync + 'static>),

    #[error("relationship mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn StdError + Send + Sync + 'static>),

    #[error("invalid relationship row")]
    InvalidRow,
}
