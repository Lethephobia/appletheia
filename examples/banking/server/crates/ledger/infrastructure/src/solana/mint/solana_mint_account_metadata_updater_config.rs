use std::sync::Arc;

use solana_sdk::signature::Keypair;

/// Configuration for `SolanaMintAccountMetadataUpdater`.
pub struct SolanaMintAccountMetadataUpdaterConfig {
    payer: Arc<Keypair>,
    mint_authority: Arc<Keypair>,
}

impl SolanaMintAccountMetadataUpdaterConfig {
    pub fn new(payer: Arc<Keypair>, mint_authority: Arc<Keypair>) -> Self {
        Self {
            payer,
            mint_authority,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn mint_authority(&self) -> &Arc<Keypair> {
        &self.mint_authority
    }
}
