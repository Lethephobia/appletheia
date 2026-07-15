use thiserror::Error;

/// Backend errors for the Solana token-deposit verifier.
#[derive(Debug, Error)]
pub enum SolanaTokenDepositVerifierError {
    #[error("pool token deposit receipt account is invalid")]
    InvalidReceipt(#[source] anchor_lang::error::Error),

    #[error("pool token deposit receipt version {0} is unsupported")]
    UnsupportedReceiptVersion(u8),
}
