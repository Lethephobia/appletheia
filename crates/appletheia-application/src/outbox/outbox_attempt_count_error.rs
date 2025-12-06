use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutboxAttemptCountError {
    #[error("outbox attempt count must be non-negative, got {0}")]
    NegativeValue(i64),
    #[error("outbox attempt count overflow")]
    Overflow,
}
