use thiserror::Error;

/// Describes why a currency pool account address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyPoolAccountAddressError {
    #[error("currency pool account address cannot be empty")]
    Empty,

    #[error("currency pool account address has an invalid format")]
    InvalidFormat,
}
