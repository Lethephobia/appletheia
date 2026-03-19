use thiserror::Error;

/// Describes why a currency symbol is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencySymbolError {
    #[error("currency symbol cannot be empty")]
    Empty,

    #[error("currency symbol is too long")]
    TooLong,

    #[error("currency symbol must be ASCII alphanumeric")]
    InvalidFormat,
}
