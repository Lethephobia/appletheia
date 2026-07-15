use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::signer::SignerError;
use thiserror::Error;

/// Backend errors for the Solana pool token deposit preparer.
#[derive(Debug, Error)]
pub enum SolanaTokenDepositPreparerError {
    #[error("invalid {kind} pubkey: {value}")]
    InvalidPubkey { kind: &'static str, value: String },

    #[error("deposit amount exceeds the SPL token transfer limit")]
    AmountOverflow,

    #[error("failed to fetch the latest blockhash")]
    Rpc(#[source] SolanaRpcClientError),

    #[error("failed to partially sign deposit transaction")]
    SignTransaction(#[source] SignerError),

    #[error("failed to serialize deposit transaction")]
    SerializeTransaction(#[source] bincode::Error),
}
