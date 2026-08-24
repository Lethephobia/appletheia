use thiserror::Error;

/// Errors produced while parsing an EVM address.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum EvmAddressError {
    #[error("EVM address must not be empty")]
    Empty,
    #[error("EVM address must contain 20 hexadecimal bytes")]
    InvalidFormat,
}
