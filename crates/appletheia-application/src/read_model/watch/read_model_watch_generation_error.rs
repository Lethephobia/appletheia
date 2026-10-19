use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchGenerationError {
    #[error("read model watch generation must be non-negative, got {0}")]
    NegativeValue(i64),

    #[error("read model watch generation overflow")]
    Overflow,
}
