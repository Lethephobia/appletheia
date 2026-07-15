use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

/// Configuration for `SolanaOnchainConfigurer`.
pub struct SolanaOnchainConfigurerConfig {
    payer: Arc<Keypair>,
    operator: Pubkey,
    upgrade_authority: Arc<Keypair>,
    program_id: Pubkey,
}

impl SolanaOnchainConfigurerConfig {
    pub fn new(
        payer: Arc<Keypair>,
        operator: Pubkey,
        upgrade_authority: Arc<Keypair>,
        program_id: Pubkey,
    ) -> Self {
        Self {
            payer,
            operator,
            upgrade_authority,
            program_id,
        }
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn operator(&self) -> &Pubkey {
        &self.operator
    }

    pub fn upgrade_authority(&self) -> &Arc<Keypair> {
        &self.upgrade_authority
    }

    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }
}
