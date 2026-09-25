use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::account::{
    PoolAuthority, WithdrawalSettlementReceipt, WithdrawalSettlementReceiptInitialization,
};
use crate::instruction_handler::{
    WithdrawalSettleInstructionAccounts, WithdrawalSettleInstructionError,
};

pub(crate) struct WithdrawalSettleInstructionHandler;

impl WithdrawalSettleInstructionHandler {
    fn transfer_from_pool<'context, 'info>(
        ctx: &Context<'context, WithdrawalSettleInstructionAccounts<'info>>,
        token_amount: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let accounts = TransferChecked {
            from: ctx.accounts.pool_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let cpi_context =
            CpiContext::new_with_signer(ctx.accounts.token_program.key(), accounts, signer_seeds);

        transfer_checked(cpi_context, token_amount, ctx.accounts.mint.decimals)
    }

    pub(crate) fn handle(
        ctx: Context<WithdrawalSettleInstructionAccounts>,
        _withdrawal_id: [u8; 16],
        token_amount: u64,
    ) -> Result<()> {
        if ctx.accounts.withdrawal_settlement_receipt.is_initialized() {
            require!(
                ctx.accounts.withdrawal_settlement_receipt.version
                    == WithdrawalSettlementReceipt::VERSION
                    && ctx.accounts.withdrawal_settlement_receipt.mint == ctx.accounts.mint.key()
                    && ctx
                        .accounts
                        .withdrawal_settlement_receipt
                        .token_account_owner
                        == ctx.accounts.token_account_owner.key()
                    && ctx.accounts.withdrawal_settlement_receipt.token_amount == token_amount,
                WithdrawalSettleInstructionError::WithdrawalSettlementReceiptConflict
            );

            return Ok(());
        }

        let authority_seeds = &[PoolAuthority::SEED, &[ctx.bumps.pool_authority]];
        Self::transfer_from_pool(&ctx, token_amount, &[authority_seeds])?;

        ctx.accounts.withdrawal_settlement_receipt.initialize(
            WithdrawalSettlementReceiptInitialization {
                mint: ctx.accounts.mint.key(),
                token_account_owner: ctx.accounts.token_account_owner.key(),
                token_amount,
                bump: ctx.bumps.withdrawal_settlement_receipt,
            },
        );

        Ok(())
    }
}
