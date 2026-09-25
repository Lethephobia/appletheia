use solana_sdk::pubkey::Pubkey;

pub struct DefaultSolanaDepositSettlementVerifierConfig {
    pub(super) program_id: Pubkey,
}

impl DefaultSolanaDepositSettlementVerifierConfig {
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
}
