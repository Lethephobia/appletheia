use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchLeaseRefreshIntervalError {
    #[error("duration must be greater than zero")]
    NonPositive,
}
