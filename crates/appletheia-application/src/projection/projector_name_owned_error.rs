use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectorNameOwnedError {
    #[error("projector name is empty")]
    Empty,

    #[error("projector name is too long: {len} (max {max})")]
    TooLong { len: usize, max: usize },

    #[error("projector name must be snake_case ascii [a-z0-9_], got {value}")]
    InvalidFormat { value: String },
}
