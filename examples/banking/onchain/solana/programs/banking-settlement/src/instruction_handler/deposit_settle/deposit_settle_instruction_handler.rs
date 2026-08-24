use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::account::{DepositSettlementReceipt, DepositSettlementReceiptInitialization};
use crate::instruction_handler::{DepositSettleInstructionAccounts, DepositSettleInstructionError};

pub(crate) struct DepositSettleInstructionHandler;

impl DepositSettleInstructionHandler {
    fn transfer_to_pool<'context, 'info>(
        ctx: &Context<'context, DepositSettleInstructionAccounts<'info>>,
        token_amount: u64,
    ) -> Result<()> {
        let accounts = TransferChecked {
            from: ctx.accounts.source_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.token_account_owner.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.token_program.key(), accounts);

        transfer_checked(cpi_context, token_amount, ctx.accounts.mint.decimals)
    }

    pub(crate) fn handle(
        ctx: Context<DepositSettleInstructionAccounts>,
        _deposit_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        if ctx.accounts.deposit_settlement_receipt.is_initialized() {
            require!(
                ctx.accounts.deposit_settlement_receipt.version
                    == DepositSettlementReceipt::VERSION
                    && ctx.accounts.deposit_settlement_receipt.mint == ctx.accounts.mint.key()
                    && ctx.accounts.deposit_settlement_receipt.pool_token_account
                        == ctx.accounts.pool_token_account.key()
                    && ctx.accounts.deposit_settlement_receipt.token_account_owner
                        == ctx.accounts.token_account_owner.key()
                    && ctx.accounts.deposit_settlement_receipt.token_amount == token_amount,
                DepositSettleInstructionError::DepositSettlementReceiptConflict
            );

            return Ok(());
        }

        Self::transfer_to_pool(&ctx, token_amount)?;

        ctx.accounts.deposit_settlement_receipt.initialize(
            DepositSettlementReceiptInitialization {
                mint: ctx.accounts.mint.key(),
                pool_token_account: ctx.accounts.pool_token_account.key(),
                token_account_owner: ctx.accounts.token_account_owner.key(),
                token_amount,
                bump: ctx.bumps.deposit_settlement_receipt,
            },
        );

        Ok(())
    }
}
