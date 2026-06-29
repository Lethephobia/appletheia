use thiserror::Error;

/// Describes why a token account owner address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum TokenAccountOwnerAddressError {
    #[error("token account owner address cannot be empty")]
    Empty,

    #[error("token account owner address has an invalid format")]
    InvalidFormat,
}
