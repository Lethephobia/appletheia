use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyDescriptionError {
    #[error("currency description cannot be empty")]
    Empty,
    #[error("currency description is too long")]
    TooLong,
}
