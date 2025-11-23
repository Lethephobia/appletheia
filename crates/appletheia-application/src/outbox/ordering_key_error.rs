use thiserror::Error;

use crate::event::{AggregateIdOwnedError, AggregateTypeOwnedError};

#[derive(Debug, Error)]
pub enum OrderingKeyError {
    #[error("ordering key must contain ':' separator")]
    MissingSeparator,
    #[error("invalid aggregate type in ordering key")]
    InvalidAggregateType(#[source] AggregateTypeOwnedError),
    #[error("invalid aggregate id in ordering key")]
    InvalidAggregateId(#[source] AggregateIdOwnedError),
}
