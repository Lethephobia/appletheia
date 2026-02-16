use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelationNameError {
    #[error("relation name is empty")]
    Empty,

    #[error("relation name is too long (len={len}, max={max})")]
    TooLong { len: usize, max: usize },

    #[error("relation name has invalid format: {value}")]
    InvalidFormat { value: String },
}

