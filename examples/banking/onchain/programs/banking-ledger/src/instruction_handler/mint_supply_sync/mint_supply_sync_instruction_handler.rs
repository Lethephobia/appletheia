use anchor_lang::prelude::*;
use anchor_spl::token_2022::{burn_checked, mint_to_checked, BurnChecked, MintToChecked};
use appletheia_anchor::instruction::InstructionHandler;

use crate::account::ProgramAuthority;
use crate::instruction_handler::{
    MintSupplySyncInstructionAccounts, MintSupplySyncInstructionArgs,
};

pub(crate) struct MintSupplySyncInstructionHandler;

impl MintSupplySyncInstructionHandler {
    fn mint_to_pool<'context, 'info>(
        ctx: &Context<'context, MintSupplySyncInstructionAccounts<'info>>,
        amount: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = MintToChecked {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_context =
            CpiContext::new_with_signer(ctx.accounts.token_program.key(), accounts, signer_seeds);

        mint_to_checked(cpi_context, amount, ctx.accounts.mint.decimals)
    }

    fn burn_from_pool<'context, 'info>(
        ctx: &Context<'context, MintSupplySyncInstructionAccounts<'info>>,
        amount: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = BurnChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_context =
            CpiContext::new_with_signer(ctx.accounts.token_program.key(), accounts, signer_seeds);

        burn_checked(cpi_context, amount, ctx.accounts.mint.decimals)
    }
}

impl InstructionHandler for MintSupplySyncInstructionHandler {
    type Accounts<'info> = MintSupplySyncInstructionAccounts<'info>;
    type Args = MintSupplySyncInstructionArgs;

    fn handle<'context, 'info>(
        ctx: Context<'context, Self::Accounts<'info>>,
        args: Self::Args,
    ) -> Result<()> {
        let current_supply = ctx.accounts.mint.supply;
        let target_supply = args.target_supply;

        if current_supply == target_supply {
            return Ok(());
        }

        let program_authority_bump = [ctx.accounts.mint_state.program_authority_bump];
        let authority_seeds = &[ProgramAuthority::SEED, &program_authority_bump];
        let signer_seeds = &[authority_seeds.as_slice()];

        if current_supply < target_supply {
            Self::mint_to_pool(&ctx, target_supply - current_supply, signer_seeds)
        } else {
            Self::burn_from_pool(&ctx, current_supply - target_supply, signer_seeds)
        }
    }
}
