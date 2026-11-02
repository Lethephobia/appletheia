use std::sync::Arc;

use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct DefaultSolanaDepositSettlementPreparerConfig {
    pub program_id: Pubkey,
    pub payer: Arc<Keypair>,
    pub operator: Arc<Keypair>,
    pub compute_unit_limit: u32,
    pub loaded_accounts_data_size_limit: u32,
}
