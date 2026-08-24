use thiserror::Error;

/// Errors produced while parsing a Solana account address.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SolanaAccountAddressError {
    #[error("Solana account address must not be empty")]
    Empty,
    #[error("Solana account address is not valid base58")]
    InvalidEncoding,
    #[error("Solana account address must decode to 32 bytes")]
    InvalidByteLength,
}
