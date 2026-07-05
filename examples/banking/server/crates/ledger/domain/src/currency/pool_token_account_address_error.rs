use thiserror::Error;

/// Describes why a currency pool token account address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PoolTokenAccountAddressError {
    #[error("currency pool token account address cannot be empty")]
    Empty,

    #[error("currency pool token account address has an invalid format")]
    InvalidFormat,
}
