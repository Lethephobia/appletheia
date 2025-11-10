use std::error::Error;
use std::fmt::Debug;
use thiserror::Error;

use crate::aggregate::Aggregate;

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("mapping failed: {0}")]
    MappingFailed(#[source] Box<dyn Error + Send + Sync + 'static>),

    #[error("aggregate error: {0}")]
    Aggregate(#[source] A::Error),

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}
