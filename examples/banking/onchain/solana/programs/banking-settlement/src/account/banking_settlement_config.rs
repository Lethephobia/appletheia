use anchor_lang::prelude::*;

pub mod banking_settlement_config_initialization;
pub mod banking_settlement_config_version;

pub use banking_settlement_config_initialization::BankingSettlementConfigInitialization;
pub use banking_settlement_config_version::BankingSettlementConfigVersion;

#[account]
pub struct BankingSettlementConfig {
    pub version: BankingSettlementConfigVersion,
    pub operator: Pubkey,
    pub bump: u8,
}

impl BankingSettlementConfig {
    pub const SEED: &[u8] = b"banking_settlement_config";
    pub const VERSION: BankingSettlementConfigVersion = BankingSettlementConfigVersion::V1;
    pub const LEN: usize = 1 + 32 + 1;

    pub fn initialize(&mut self, initialization: BankingSettlementConfigInitialization) {
        let BankingSettlementConfigInitialization { operator, bump } = initialization;

        self.version = Self::VERSION;
        self.operator = operator;
        self.bump = bump;
    }

    pub fn is_initialized(&self) -> bool {
        match self.version {
            BankingSettlementConfigVersion::Uninitialized => false,
            BankingSettlementConfigVersion::V1 => true,
        }
    }

    pub fn change_operator(&mut self, operator: Pubkey) {
        self.operator = operator;
    }
}
