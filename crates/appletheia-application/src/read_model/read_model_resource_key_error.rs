use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelResourceKeyError {
    #[error("read model resource key is empty")]
    Empty,

    #[error("read model resource key is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },

    #[error("read model resource key must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },

    #[error("reserved resource field name: {0}")]
    ReservedName(String),
}
