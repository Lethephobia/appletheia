use thiserror::Error;

use super::{AggregateId, AggregateVersion, AggregateVersionError};

#[derive(Debug, Error)]
pub enum AggregateError<A: AggregateId> {
    #[error("invalid aggregate id: {0}, expected {1}")]
    InvalidAggregateId(A, A),

    #[error("aggregate version error: {0}")]
    Version(#[source] AggregateVersionError),

    #[error("invalid next event version: {0}, expected {1}")]
    InvalidNextEventVersion(AggregateVersion, AggregateVersion),

    #[error("no state")]
    NoState,
}
