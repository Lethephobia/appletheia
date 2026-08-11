use std::error::Error;

use thiserror::Error;

/// Reports a failure to durably enqueue persisted events.
#[derive(Debug, Error)]
pub enum EventOutboxEnqueueError {
    #[error("not in transaction")]
    NotInTransaction,
    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
