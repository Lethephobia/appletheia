use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchEndpointLeaseDurationError {
    #[error("duration must be greater than zero")]
    NonPositive,
}
