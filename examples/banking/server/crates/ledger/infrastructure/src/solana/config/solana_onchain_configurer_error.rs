use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::signer::SignerError;
use thiserror::Error;

/// Represents Solana adapter errors while configuring the on-chain ledger backend.
#[derive(Debug, Error)]
pub enum SolanaOnchainConfigurerError {
    #[error("Solana on-chain configuration transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
