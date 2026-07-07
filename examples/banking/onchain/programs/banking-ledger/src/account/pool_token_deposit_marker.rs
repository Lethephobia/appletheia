use anchor_lang::prelude::*;

pub mod pool_token_deposit_marker_initialization;

pub use pool_token_deposit_marker_initialization::PoolTokenDepositMarkerInitialization;

#[account]
pub struct PoolTokenDepositMarker {
    pub version: u8,
    pub mint_id: [u8; 16],
    pub token_account_owner: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl PoolTokenDepositMarker {
    pub const SEED: &[u8] = b"pool_token_deposit_marker";
    pub const VERSION: u8 = 1;
    pub const LEN: usize = 1 + 16 + 32 + 8 + 1;

    pub fn initialize(&mut self, initialization: PoolTokenDepositMarkerInitialization) {
        let PoolTokenDepositMarkerInitialization {
            mint_id,
            token_account_owner,
            amount,
            bump,
        } = initialization;

        self.version = Self::VERSION;
        self.mint_id = mint_id;
        self.token_account_owner = token_account_owner;
        self.amount = amount;
        self.bump = bump;
    }

    pub fn is_initialized(&self) -> bool {
        self.version != 0
    }
}
