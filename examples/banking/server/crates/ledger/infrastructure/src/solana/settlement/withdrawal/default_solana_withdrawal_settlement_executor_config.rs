use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct DefaultSolanaWithdrawalSettlementExecutorConfig {
    pub(super) program_id: Pubkey,
    pub(super) payer: Arc<Keypair>,
    pub(super) operator: Arc<Keypair>,
}

impl DefaultSolanaWithdrawalSettlementExecutorConfig {
    pub fn new(program_id: Pubkey, payer: Arc<Keypair>, operator: Arc<Keypair>) -> Self {
        Self {
            program_id,
            payer,
            operator,
        }
    }
}
