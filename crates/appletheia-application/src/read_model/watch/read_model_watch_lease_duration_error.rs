use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchLeaseDurationError {
    #[error("duration must be greater than zero")]
    NonPositive,
}
