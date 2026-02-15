use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectKindError {
    #[error("subject kind is empty")]
    Empty,

    #[error("subject kind is too long (len={len}, max={max})")]
    TooLong { len: usize, max: usize },

    #[error("subject kind must be snake_case ascii: [a-z0-9_] (value={value})")]
    InvalidFormat { value: String },
}
