use anchor_lang::prelude::*;

pub struct BankingLedgerConfigInitialization {
    pub operator: Pubkey,
    pub bump: u8,
}
