use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

/// Configuration for `SolanaMintAccountCreator`.
pub struct SolanaMintAccountCreatorConfig {
    payer: Arc<Keypair>,
    mint_authority: Arc<Keypair>,
    pool_account_owner: Arc<Keypair>,
    freeze_authority: Option<Pubkey>,
}

impl SolanaMintAccountCreatorConfig {
    pub fn new(
        payer: Arc<Keypair>,
        mint_authority: Arc<Keypair>,
        pool_account_owner: Arc<Keypair>,
        freeze_authority: Option<Pubkey>,
    ) -> Self {
        Self {
            payer,
            mint_authority,
            pool_account_owner,
            freeze_authority,
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

    pub fn freeze_authority(&self) -> Option<Pubkey> {
        self.freeze_authority
    }
}
