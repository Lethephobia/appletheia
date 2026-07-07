use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use appletheia_anchor::instruction::InstructionHandler;

use crate::account::{
    PoolTokenTransferMarker, PoolTokenTransferMarkerInitialization, ProgramAuthority,
};
use crate::instruction_handler::{
    PoolTokenTransferInstructionAccounts, PoolTokenTransferInstructionArgs,
    PoolTokenTransferInstructionError,
};

pub(crate) struct PoolTokenTransferInstructionHandler;

impl PoolTokenTransferInstructionHandler {
    fn transfer_from_pool<'context, 'info>(
        ctx: &Context<'context, PoolTokenTransferInstructionAccounts<'info>>,
        amount: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = TransferChecked {
            from: ctx.accounts.pool_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_context =
            CpiContext::new_with_signer(ctx.accounts.token_program.key(), accounts, signer_seeds);

        transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)
    }
}

impl InstructionHandler for PoolTokenTransferInstructionHandler {
    type Accounts<'info> = PoolTokenTransferInstructionAccounts<'info>;
    type Args = PoolTokenTransferInstructionArgs;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        args: Self::Args,
    ) -> Result<()> {
        let PoolTokenTransferInstructionArgs {
            idempotency_key: _,
            mint_id,
            amount,
        } = args;

        if ctx.accounts.pool_token_transfer_marker.is_initialized() {
            require!(
                ctx.accounts.pool_token_transfer_marker.version == PoolTokenTransferMarker::VERSION
                    && ctx.accounts.pool_token_transfer_marker.mint_id == mint_id
                    && ctx.accounts.pool_token_transfer_marker.token_account_owner
                        == ctx.accounts.token_account_owner.key()
                    && ctx.accounts.pool_token_transfer_marker.amount == amount,
                PoolTokenTransferInstructionError::PoolTokenTransferMarkerConflict
            );

            return Ok(());
        }

        ctx.accounts
            .pool_token_transfer_marker
            .initialize(PoolTokenTransferMarkerInitialization {
                mint_id,
                token_account_owner: ctx.accounts.token_account_owner.key(),
                amount,
                bump: ctx.bumps.pool_token_transfer_marker,
            });

        let authority_seeds = &[
            ProgramAuthority::SEED,
            &[ctx.accounts.mint_state.program_authority_bump],
        ];
        Self::transfer_from_pool(&ctx, amount, &[authority_seeds])
    }
}
