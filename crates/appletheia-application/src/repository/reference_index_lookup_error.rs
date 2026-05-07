use std::error::Error;

use thiserror::Error;

/// Errors returned by `ReferenceIndexLookup`.
#[derive(Debug, Error)]
pub enum ReferenceIndexLookupError {
    #[error("invalid source aggregate id: {0}")]
    SourceAggregateId(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
