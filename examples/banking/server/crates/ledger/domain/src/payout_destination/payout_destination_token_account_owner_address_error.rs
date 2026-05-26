use thiserror::Error;

/// Describes why a payout destination token account owner address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PayoutDestinationTokenAccountOwnerAddressError {
    #[error("payout destination token account owner address cannot be empty")]
    Empty,

    #[error("payout destination token account owner address has an invalid format")]
    InvalidFormat,
}
