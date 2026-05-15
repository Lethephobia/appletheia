use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectionConsistencyTimeoutError {
    #[error("duration must be non-negative")]
    Negative,

    #[error("duration is out of range for std::time::Duration")]
    OutOfRange,
}
