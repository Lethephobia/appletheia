use std::sync::Arc;

use solana_sdk::signature::Keypair;

/// Configuration for `SolanaMintSupplySynchronizer`.
pub struct SolanaMintSupplySynchronizerConfig {
    payer: Arc<Keypair>,
    mint_authority: Arc<Keypair>,
    pool_account_owner: Arc<Keypair>,
}

impl SolanaMintSupplySynchronizerConfig {
    pub fn new(
        payer: Arc<Keypair>,
        mint_authority: Arc<Keypair>,
        pool_account_owner: Arc<Keypair>,
    ) -> Self {
        Self {
            payer,
            mint_authority,
            pool_account_owner,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn mint_authority(&self) -> &Arc<Keypair> {
        &self.mint_authority
    }

    pub fn pool_account_owner(&self) -> &Arc<Keypair> {
        &self.pool_account_owner
    }
}
