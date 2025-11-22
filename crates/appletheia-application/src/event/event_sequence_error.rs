use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventSequenceError {
    #[error("event sequence must be non-negative, got {0}")]
    NegativeValue(i64),
}
