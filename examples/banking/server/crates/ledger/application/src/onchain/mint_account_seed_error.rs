use thiserror::Error;

/// Describes why a mint account seed is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintAccountSeedError {
    #[error("mint account seed cannot be empty")]
    Empty,

    #[error("mint account seed is too long")]
    TooLong,

    #[error("mint account seed has an invalid format")]
    InvalidFormat,
}
