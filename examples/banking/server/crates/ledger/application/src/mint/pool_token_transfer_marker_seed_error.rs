use thiserror::Error;

/// Describes why a pool token transfer marker seed is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PoolTokenTransferMarkerSeedError {
    #[error("pool token transfer marker seed cannot be empty")]
    Empty,

    #[error("pool token transfer marker seed is too long")]
    TooLong,

    #[error("pool token transfer marker seed has an invalid format")]
    InvalidFormat,
}
