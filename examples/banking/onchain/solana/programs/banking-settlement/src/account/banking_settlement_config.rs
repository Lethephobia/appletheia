use anchor_lang::prelude::*;

pub mod banking_settlement_config_initialization;

pub use banking_settlement_config_initialization::BankingSettlementConfigInitialization;

#[account]
pub struct BankingSettlementConfig {
    pub version: u8,
    pub operator: Pubkey,
    pub bump: u8,
}

impl BankingSettlementConfig {
    pub const SEED: &[u8] = b"banking_settlement_config";
    pub const VERSION: u8 = 1;
    pub const LEN: usize = 1 + 32 + 1;

    pub fn initialize(&mut self, initialization: BankingSettlementConfigInitialization) {
        let BankingSettlementConfigInitialization { operator, bump } = initialization;

        self.version = Self::VERSION;
        self.operator = operator;
        self.bump = bump;
    }

    pub fn is_initialized(&self) -> bool {
        self.version != 0
    }

    pub fn change_operator(&mut self, operator: Pubkey) {
        self.operator = operator;
    }
}
