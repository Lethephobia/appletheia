use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchLeaseDurationError {
    #[error("watch endpoint lease duration must be positive")]
    NonPositive,
}
