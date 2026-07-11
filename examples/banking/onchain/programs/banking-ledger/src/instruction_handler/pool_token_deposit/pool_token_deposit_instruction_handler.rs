use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use banking_anchor::instruction::InstructionHandler;

use crate::account::{PoolTokenDepositMarker, PoolTokenDepositMarkerInitialization};
use crate::instruction_handler::{
    PoolTokenDepositInstructionAccounts, PoolTokenDepositInstructionArgs,
    PoolTokenDepositInstructionError,
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
}

impl InstructionHandler for PoolTokenDepositInstructionHandler {
    type Accounts<'info> = PoolTokenDepositInstructionAccounts<'info>;
    type Args = PoolTokenDepositInstructionArgs;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        args: Self::Args,
    ) -> Result<()> {
        let PoolTokenDepositInstructionArgs {
            idempotency_key: _,
            mint_id,
            amount,
        } = args;

        if ctx.accounts.pool_token_deposit_marker.is_initialized() {
            require!(
                ctx.accounts.pool_token_deposit_marker.version == PoolTokenDepositMarker::VERSION
                    && ctx.accounts.pool_token_deposit_marker.mint_id == mint_id
                    && ctx.accounts.pool_token_deposit_marker.token_account_owner
                        == ctx.accounts.token_account_owner.key()
                    && ctx.accounts.pool_token_deposit_marker.amount == amount,
                PoolTokenDepositInstructionError::PoolTokenDepositMarkerConflict
            );

            return Ok(());
        }

        Self::transfer_to_pool(&ctx, amount)?;

        ctx.accounts
            .pool_token_deposit_marker
            .initialize(PoolTokenDepositMarkerInitialization {
                mint_id,
                token_account_owner: ctx.accounts.token_account_owner.key(),
                amount,
                bump: ctx.bumps.pool_token_deposit_marker,
            });

        Ok(())
    }
}
