use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaNameOwnedError {
    #[error("saga name is empty")]
    Empty,

    #[error("saga name is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },

    #[error("saga name must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },
}
