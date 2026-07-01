use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::signer::SignerError;
use thiserror::Error;

/// Represents Solana adapter errors while synchronizing mint supply.
#[derive(Debug, Error)]
pub enum SolanaMintSupplySynchronizerError {
    #[error("Solana target supply exceeds the token program limit")]
    TargetSupplyOverflow,

    #[error("Solana transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
