use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelTypeOwnedError {
    #[error("read model type is empty")]
    Empty,

    #[error("read model type is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },

    #[error("read model type must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },
}
