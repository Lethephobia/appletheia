use thiserror::Error;

/// Error returned when a page limit is invalid.
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum PageLimitError {
    #[error("page limit must be greater than zero")]
    Zero,

    #[error("page limit must be less than or equal to {max}, but was {actual}")]
    TooLarge { max: u32, actual: u32 },
}
