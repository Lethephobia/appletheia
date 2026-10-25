use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointLeaseRefreshIntervalError {
    #[error("duration must be greater than zero")]
    NonPositive,
}
