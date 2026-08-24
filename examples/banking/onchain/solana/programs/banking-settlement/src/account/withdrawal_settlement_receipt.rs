use anchor_lang::prelude::*;

pub mod withdrawal_settlement_receipt_initialization;

pub use withdrawal_settlement_receipt_initialization::WithdrawalSettlementReceiptInitialization;

#[account]
pub struct WithdrawalSettlementReceipt {
    pub version: u8,
    pub mint: Pubkey,
    pub token_account_owner: Pubkey,
    pub token_amount: u64,
    pub bump: u8,
}

impl WithdrawalSettlementReceipt {
    pub const SEED: &[u8] = b"withdrawal_settlement_receipt";
    pub const VERSION: u8 = 1;
    pub const LEN: usize = 1 + 32 + 32 + 8 + 1;

    pub fn initialize(&mut self, initialization: WithdrawalSettlementReceiptInitialization) {
        let WithdrawalSettlementReceiptInitialization {
            mint,
            token_account_owner,
            token_amount,
            bump,
        } = initialization;

        self.version = Self::VERSION;
        self.mint = mint;
        self.token_account_owner = token_account_owner;
        self.token_amount = token_amount;
        self.bump = bump;
    }

    pub fn is_initialized(&self) -> bool {
        self.version != 0
    }
}
