use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadYourWritesTimeoutError {
    #[error("duration must be non-negative")]
    Negative,

    #[error("duration is out of range for std::time::Duration")]
    OutOfRange,
}

