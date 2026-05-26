use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

/// Configuration for `SolanaMintAccountCreator`.
pub struct SolanaMintAccountCreatorConfig {
    payer: Arc<Keypair>,
    mint_authority: Arc<Keypair>,
    pool_token_account_owner_address: Pubkey,
    freeze_authority: Option<Pubkey>,
}

impl SolanaMintAccountCreatorConfig {
    pub fn new(
        payer: Arc<Keypair>,
        mint_authority: Arc<Keypair>,
        pool_token_account_owner_address: Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Self {
        Self {
            payer,
            mint_authority,
            pool_token_account_owner_address,
            freeze_authority,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn mint_authority(&self) -> &Arc<Keypair> {
        &self.mint_authority
    }

    pub fn pool_token_account_owner_address(&self) -> &Pubkey {
        &self.pool_token_account_owner_address
    }

    pub fn freeze_authority(&self) -> Option<Pubkey> {
        self.freeze_authority
    }
}
