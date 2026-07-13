use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

/// Configuration for `SolanaTokenDepositPreparer`.
#[derive(Clone)]
pub struct SolanaTokenDepositPreparerConfig {
    program_id: Pubkey,
    payer: Arc<Keypair>,
    operator: Arc<Keypair>,
}

impl SolanaTokenDepositPreparerConfig {
    pub fn new(program_id: Pubkey, payer: Arc<Keypair>, operator: Arc<Keypair>) -> Self {
        Self {
            program_id,
            payer,
            operator,
        }
    }

    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }

    pub fn payer(&self) -> &Arc<Keypair> {
        &self.payer
    }

    pub fn operator(&self) -> &Arc<Keypair> {
        &self.operator
    }
}
