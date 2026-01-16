use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventOutboxFetcherError {
    #[error("outbox mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("outbox persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("transaction is not active")]
    NotInTransaction,
}
