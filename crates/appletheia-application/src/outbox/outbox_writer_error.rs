use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxWriterError {
    #[error("outbox persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
