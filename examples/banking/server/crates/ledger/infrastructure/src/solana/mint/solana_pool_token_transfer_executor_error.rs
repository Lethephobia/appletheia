use thiserror::Error;

/// Backend errors for the Solana pool token transfer executor.
#[derive(Debug, Error)]
pub enum SolanaPoolTokenTransferExecutorError {
    #[error("invalid {kind} pubkey: {value}")]
    InvalidPubkey { kind: &'static str, value: String },

    #[error("failed to derive marker account address")]
    MarkerAccountAddress(#[source] solana_sdk::pubkey::PubkeyError),

    #[error("no signature was found for marker account {marker_account_address}")]
    MarkerAccountSignatureMissing { marker_account_address: String },

    #[error("failed to sign pool token transfer transaction")]
    SignTransaction(#[source] solana_sdk::signer::SignerError),

    #[error(
        "provided pool token account address does not match the configured owner ATA: expected {expected}, got {provided}"
    )]
    PoolTokenAccountAddressMismatch { expected: String, provided: String },

    #[error("withdrawal amount exceeds the SPL token transfer limit")]
    AmountOverflow,

    #[error("failed to build on-chain transaction id from signature: {signature}")]
    InvalidOnchainTransactionId { signature: String },

    #[error("failed to build SPL token transfer instruction")]
    TransferInstruction(#[source] solana_sdk::program_error::ProgramError),
}
