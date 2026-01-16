use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandOutboxFetcherError {
    #[error("command outbox mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("command outbox persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("transaction is not active")]
    NotInTransaction,
}
