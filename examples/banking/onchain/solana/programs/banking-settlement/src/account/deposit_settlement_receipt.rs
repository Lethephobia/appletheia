use anchor_lang::prelude::*;

pub mod deposit_settlement_receipt_initialization;

pub use deposit_settlement_receipt_initialization::DepositSettlementReceiptInitialization;

#[account]
pub struct DepositSettlementReceipt {
    pub version: u8,
    pub mint: Pubkey,
    pub pool_token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub token_amount: u64,
    pub bump: u8,
}

impl DepositSettlementReceipt {
    pub const SEED: &[u8] = b"deposit_settlement_receipt";
    pub const VERSION: u8 = 1;
    pub const LEN: usize = 1 + 32 + 32 + 32 + 8 + 1;

    pub fn initialize(&mut self, initialization: DepositSettlementReceiptInitialization) {
        let DepositSettlementReceiptInitialization {
            mint,
            pool_token_account,
            token_account_owner,
            token_amount,
            bump,
        } = initialization;

        self.version = Self::VERSION;
        self.mint = mint;
        self.pool_token_account = pool_token_account;
        self.token_account_owner = token_account_owner;
        self.token_amount = token_amount;
        self.bump = bump;
    }

    pub fn is_initialized(&self) -> bool {
        self.version != 0
    }
}
