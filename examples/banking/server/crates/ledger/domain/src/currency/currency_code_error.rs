use thiserror::Error;

/// Describes why a currency code is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyCodeError {
    #[error("currency code must contain one or more uppercase ASCII letters")]
    InvalidFormat,
}
