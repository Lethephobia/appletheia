use thiserror::Error;

/// Describes why a currency mint token program ID is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyTokenProgramIdError {
    #[error("currency mint token program ID cannot be empty")]
    Empty,

    #[error("currency mint token program ID has an invalid format")]
    InvalidFormat,
}
