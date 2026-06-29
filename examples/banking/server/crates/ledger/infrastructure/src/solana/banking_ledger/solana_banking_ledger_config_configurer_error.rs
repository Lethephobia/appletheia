use solana_client::client_error::ClientError as SolanaRpcClientError;
use solana_sdk::signer::SignerError;
use thiserror::Error;

/// Represents Solana adapter errors while configuring the banking ledger config.
#[derive(Debug, Error)]
pub enum SolanaBankingLedgerConfigConfigurerError {
    #[error("Solana banking ledger config transaction signing failed")]
    Sign(#[from] SignerError),

    #[error("Solana RPC failed")]
    Rpc(#[from] SolanaRpcClientError),
}
