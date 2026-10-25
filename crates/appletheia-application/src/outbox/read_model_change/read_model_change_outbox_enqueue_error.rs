use std::error::Error;

use thiserror::Error;

/// Reports a failure to durably enqueue read model changes.
#[derive(Debug, Error)]
pub enum ReadModelChangeOutboxEnqueueError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
