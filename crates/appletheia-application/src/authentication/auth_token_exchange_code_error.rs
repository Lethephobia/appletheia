use thiserror::Error;

/// Errors that can occur when parsing an `AuthTokenExchangeCode`.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum AuthTokenExchangeCodeError {
    #[error("exchange code is too short: {length} < {min}")]
    TooShort { length: usize, min: usize },

    #[error("exchange code is too long: {length} > {max}")]
    TooLong { length: usize, max: usize },

    #[error("exchange code contains invalid character `{character}` at position {position}")]
    InvalidCharacter { character: char, position: usize },
}
