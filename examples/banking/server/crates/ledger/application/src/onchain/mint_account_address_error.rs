use thiserror::Error;

/// Describes why a mint account address returned by an on-chain gateway is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum MintAccountAddressError {
    #[error("mint account address cannot be empty")]
    Empty,

    #[error("mint account address has an invalid format")]
    InvalidFormat,
}
