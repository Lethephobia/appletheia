use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaveatNameError {
    #[error("caveat name is empty")]
    Empty,

    #[error("caveat name is too long (len={len}, max={max})")]
    TooLong { len: usize, max: usize },

    #[error("caveat name has invalid format: {value}")]
    InvalidFormat { value: String },
}

