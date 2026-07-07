use anchor_lang::prelude::*;

pub struct PoolTokenDepositMarkerInitialization {
    pub mint_id: [u8; 16],
    pub token_account_owner: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
