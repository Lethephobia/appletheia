use thiserror::Error;

use crate::aggregate::Aggregate;

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("aggregate error: {0}")]
    AggregateError(#[source] A::Error),
}
