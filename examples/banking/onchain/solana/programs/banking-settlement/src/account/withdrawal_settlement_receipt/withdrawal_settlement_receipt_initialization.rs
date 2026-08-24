use anchor_lang::prelude::*;

pub struct WithdrawalSettlementReceiptInitialization {
    pub mint: Pubkey,
    pub token_account_owner: Pubkey,
    pub token_amount: u64,
    pub bump: u8,
}
