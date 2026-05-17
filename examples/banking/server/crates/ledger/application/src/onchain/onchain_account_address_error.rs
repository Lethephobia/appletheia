use thiserror::Error;

/// Describes why an on-chain account address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum OnchainAccountAddressError {
    #[error("on-chain account address cannot be empty")]
    Empty,

    #[error("on-chain account address has an invalid format")]
    InvalidFormat,
}
