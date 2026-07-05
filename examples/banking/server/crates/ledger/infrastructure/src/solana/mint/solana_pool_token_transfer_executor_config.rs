use std::sync::Arc;

use solana_sdk::signature::Keypair;

/// Configuration for `SolanaPoolTokenTransferExecutor`.
pub struct SolanaPoolTokenTransferExecutorConfig {
    payer: Arc<Keypair>,
    pool_token_account_owner: Arc<Keypair>,
}

impl SolanaPoolTokenTransferExecutorConfig {
    pub fn new(payer: Arc<Keypair>, pool_token_account_owner: Arc<Keypair>) -> Self {
        Self {
            payer,
            pool_token_account_owner,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn pool_token_account_owner(&self) -> &Arc<Keypair> {
        &self.pool_token_account_owner
    }
}
