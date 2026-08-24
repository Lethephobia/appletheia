use thiserror::Error;

/// Errors produced while parsing an EVM transaction hash.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum EvmTransactionHashError {
    #[error("EVM transaction hash must not be empty")]
    Empty,
    #[error("EVM transaction hash must contain 32 hexadecimal bytes")]
    InvalidFormat,
}
