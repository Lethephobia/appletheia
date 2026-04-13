use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaRunStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),

    #[error("failed to map persisted saga run")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync>),

    #[error("failed to serialize saga context")]
    ContextSerialize(#[source] serde_json::Error),
}
