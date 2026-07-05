use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

/// Configuration for `SolanaMintMetadataUpdater`.
pub struct SolanaMintMetadataUpdaterConfig {
    payer: Arc<Keypair>,
    operator: Arc<Keypair>,
    program_id: Pubkey,
}

impl SolanaMintMetadataUpdaterConfig {
    pub fn new(payer: Arc<Keypair>, operator: Arc<Keypair>, program_id: Pubkey) -> Self {
        Self {
            payer,
            operator,
            program_id,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn operator(&self) -> &Arc<Keypair> {
        &self.operator
    }

    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }
}
