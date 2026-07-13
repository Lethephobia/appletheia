use solana_sdk::pubkey::Pubkey;

/// Configuration for `SolanaTokenDepositVerifier`.
#[derive(Clone, Debug)]
pub struct SolanaTokenDepositVerifierConfig {
    program_id: Pubkey,
}

impl SolanaTokenDepositVerifierConfig {
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }

    pub fn program_id(&self) -> &Pubkey {
        &self.program_id
    }
}
