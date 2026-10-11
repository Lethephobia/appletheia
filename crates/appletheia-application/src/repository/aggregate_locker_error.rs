use crate::aggregate::AggregateRef;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateLockerError {
    #[error("aggregate lock not found: {aggregate:?}")]
    NotFound { aggregate: AggregateRef },

    #[error("cannot upgrade a shared aggregate lock: {aggregate:?}")]
    SharedLockUpgrade { aggregate: AggregateRef },

    #[error("aggregate lock persistence failed: {0}")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
