use thiserror::Error;

/// Backend errors for the Solana pool token transfer executor.
#[derive(Debug, Error)]
pub enum SolanaPoolTokenTransferExecutorError {
    #[error("invalid {kind} pubkey: {value}")]
    InvalidPubkey { kind: &'static str, value: String },

    #[error("failed to sign pool token transfer transaction")]
    SignTransaction(#[source] solana_sdk::signer::SignerError),

    #[error("withdrawal amount exceeds the SPL token transfer limit")]
    AmountOverflow,
}
