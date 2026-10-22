use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointConfigError {
    #[error("endpoint lease refresh interval must be shorter than its lease duration")]
    EndpointRefreshIntervalTooLong,
}
