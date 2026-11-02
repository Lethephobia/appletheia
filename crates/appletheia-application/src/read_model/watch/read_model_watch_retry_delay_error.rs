use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchRetryDelayError {
    #[error("retry delay must be greater than zero")]
    NonPositive,
}
