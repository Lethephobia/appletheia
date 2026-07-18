use std::error::Error;

use thiserror::Error;

use crate::Retryability;

/// Errors returned by `ReferenceIndexLookup`.
#[derive(Debug, Error)]
pub enum ReferenceIndexLookupError {
    #[error("invalid source aggregate id: {0}")]
    SourceAggregateId(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl Retryability for ReferenceIndexLookupError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::SourceAggregateId(_) => false,
            Self::Persistence(_) => true,
        }
    }
}
