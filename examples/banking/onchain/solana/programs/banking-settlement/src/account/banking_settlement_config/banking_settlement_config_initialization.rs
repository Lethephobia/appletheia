use anchor_lang::prelude::*;

pub struct BankingSettlementConfigInitialization {
    pub operator: Pubkey,
    pub bump: u8,
}
