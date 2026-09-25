use std::error::Error;

use thiserror::Error;

/// Reports a failure to enqueue terminal command notifications.
#[derive(Debug, Error)]
pub enum CommandFailureOutboxEnqueueError {
    #[error("not in transaction")]
    NotInTransaction,

    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
