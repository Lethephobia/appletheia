use std::error::Error;

use thiserror::Error;

/// Describes failures while reading saga instance status.
#[derive(Debug, Error)]
pub enum SagaStatusLookupError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),

    #[error("invalid persisted saga instance: {message}")]
    InvalidPersistedInstance { message: &'static str },
}
