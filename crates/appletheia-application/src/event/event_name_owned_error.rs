use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventNameOwnedError {
    #[error("event name is empty")]
    Empty,

    #[error("event name is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },

    #[error("event name must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },
}
