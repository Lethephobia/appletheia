use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct DefaultSolanaDepositSettlementPreparerConfig {
    pub(super) program_id: Pubkey,
    pub(super) operator: Arc<Keypair>,
}

impl DefaultSolanaDepositSettlementPreparerConfig {
    pub fn new(program_id: Pubkey, operator: Arc<Keypair>) -> Self {
        Self {
            program_id,
            operator,
        }
    }
}
