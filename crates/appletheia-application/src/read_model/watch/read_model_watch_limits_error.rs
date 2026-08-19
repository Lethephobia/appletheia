use thiserror::Error;

/// Reports an invalid read-model watch resource limit.
#[derive(Debug, Error)]
pub enum ReadModelWatchLimitsError {
    #[error("read-model watch limit must be greater than zero: {0}")]
    Zero(&'static str),
}
