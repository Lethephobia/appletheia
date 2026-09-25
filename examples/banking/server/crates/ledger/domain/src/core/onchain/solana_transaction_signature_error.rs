use thiserror::Error;

/// Errors produced while parsing a Solana transaction signature.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SolanaTransactionSignatureError {
    #[error("Solana transaction signature must not be empty")]
    Empty,
    #[error("Solana transaction signature is not valid base58")]
    InvalidEncoding,
    #[error("Solana transaction signature must decode to 64 bytes")]
    InvalidByteLength,
}
