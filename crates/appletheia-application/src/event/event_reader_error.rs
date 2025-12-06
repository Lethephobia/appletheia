use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventReaderError {
    #[error("event mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("event persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
