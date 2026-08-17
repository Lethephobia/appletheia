use std::error::Error;

use thiserror::Error;

/// Reports a failure to durably enqueue projected fragment changes.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeOutboxEnqueueError {
    #[error("not in transaction")]
    NotInTransaction,
    #[error("persistence error")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
