use std::error::Error;

use thiserror::Error;

/// Errors returned by `UniqueValueOwnerLookup`.
#[derive(Debug, Error)]
pub enum UniqueValueOwnerLookupError {
    #[error("invalid owner aggregate id: {0}")]
    OwnerAggregateId(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
