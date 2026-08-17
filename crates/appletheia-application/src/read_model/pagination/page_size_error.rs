use thiserror::Error;

/// Error returned when a page size is invalid.
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum PageSizeError {
    #[error("page size must be greater than zero")]
    Zero,
    #[error("page size must be less than or equal to {max}, but was {actual}")]
    TooLarge { max: u32, actual: u32 },
}
