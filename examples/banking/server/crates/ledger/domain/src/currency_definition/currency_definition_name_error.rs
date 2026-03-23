use thiserror::Error;

/// Describes why a currency-definition name is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyDefinitionNameError {
    #[error("currency-definition name cannot be empty")]
    Empty,

    #[error("currency-definition name is too long")]
    TooLong,
}
