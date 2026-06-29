use banking_ledger_application::{MintAccountAddressError, PoolTokenAccountAddressError};
use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::signer::SignerError;
use thiserror::Error;

/// Represents Solana adapter errors while provisioning a mint.
#[derive(Debug, Error)]
pub enum SolanaMintProvisionerError {
    #[error("Solana mint account address returned by the adapter is invalid")]
    MintAccountAddress(#[source] MintAccountAddressError),

    #[error("Solana pool token account address returned by the adapter is invalid")]
    PoolTokenAccountAddress(#[source] PoolTokenAccountAddressError),

    #[error("Solana transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
