use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateTypeOwnedError {
    #[error("aggregate type is empty")]
    Empty,
    #[error("aggregate type is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },
    #[error("aggregate type must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },
}
