use thiserror::Error;

/// Describes why a pool token account address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PoolTokenAccountAddressError {
    #[error("pool token account address cannot be empty")]
    Empty,

    #[error("pool token account address has an invalid format")]
    InvalidFormat,
}
