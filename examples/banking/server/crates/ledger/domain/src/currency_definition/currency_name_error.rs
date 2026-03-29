use thiserror::Error;

/// Describes why a currency name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyNameError {
    #[error("currency name cannot be empty")]
    Empty,

    #[error("currency name is too long")]
    TooLong,
}
