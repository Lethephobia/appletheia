use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaStoreError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),

    #[error("failed to deserialize saga state")]
    StateDeserialize(#[source] serde_json::Error),

    #[error("failed to serialize saga state")]
    StateSerialize(#[source] serde_json::Error),

    #[error("invalid persisted saga instance: {message}")]
    InvalidPersistedInstance { message: &'static str },
}
