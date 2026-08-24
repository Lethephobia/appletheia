use anchor_lang::prelude::*;

pub struct DepositSettlementReceiptInitialization {
    pub mint: Pubkey,
    pub pool_token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub token_amount: u64,
    pub bump: u8,
}
