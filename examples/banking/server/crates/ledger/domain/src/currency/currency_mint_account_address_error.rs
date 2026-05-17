use thiserror::Error;

/// Describes why a currency mint account address is invalid.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum CurrencyMintAccountAddressError {
    #[error("currency mint account address cannot be empty")]
    Empty,

    #[error("currency mint account address has an invalid format")]
    InvalidFormat,
}
