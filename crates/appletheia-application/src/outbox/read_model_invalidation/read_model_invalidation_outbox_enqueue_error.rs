use std::error::Error;

use thiserror::Error;

/// Reports a failure to durably enqueue read-model invalidations.
#[derive(Debug, Error)]
pub enum ReadModelInvalidationOutboxEnqueueError {
    #[error("not in transaction")]
    NotInTransaction,
    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
