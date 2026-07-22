use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};

use crate::account::{PoolTokenDepositReceipt, PoolTokenDepositReceiptInitialization};
use crate::instruction_handler::{
    PoolTokenDepositInstructionAccounts, PoolTokenDepositInstructionError,
};

pub(crate) struct PoolTokenDepositInstructionHandler;

impl PoolTokenDepositInstructionHandler {
    fn transfer_to_pool<'context, 'info>(
        ctx: &Context<'context, PoolTokenDepositInstructionAccounts<'info>>,
        amount: u64,
    ) -> Result<()> {
        let accounts = TransferChecked {
            from: ctx.accounts.source_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.token_account_owner.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.token_program.key(), accounts);

        transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)
    }

    pub(crate) fn handle(
        ctx: Context<PoolTokenDepositInstructionAccounts>,
        _pool_token_deposit_id: [u8; 16],
        _mint_id: [u8; 16],
        amount: u64,
    ) -> Result<()> {
        if ctx.accounts.pool_token_deposit_receipt.is_initialized() {
            require!(
                ctx.accounts.pool_token_deposit_receipt.version == PoolTokenDepositReceipt::VERSION
                    && ctx.accounts.pool_token_deposit_receipt.amount == amount,
                PoolTokenDepositInstructionError::PoolTokenDepositReceiptConflict
            );

            return Ok(());
        }

        Self::transfer_to_pool(&ctx, amount)?;

        ctx.accounts
            .pool_token_deposit_receipt
            .initialize(PoolTokenDepositReceiptInitialization {
                amount,
                bump: ctx.bumps.pool_token_deposit_receipt,
            });

        Ok(())
    }
}
