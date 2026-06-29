use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::{program_error::ProgramError, signer::SignerError};
use thiserror::Error;

/// Represents Solana adapter errors while updating mint metadata.
#[derive(Debug, Error)]
pub enum SolanaMintAccountMetadataUpdaterError {
    #[error("Solana mint account data is invalid")]
    MintAccountData(#[source] ProgramError),

    #[error("Solana transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
