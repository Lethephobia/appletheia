use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::account::{BankingSettlementConfig, DepositSettlementReceipt, PoolAuthority};
use crate::instruction_handler::DepositSettleInstructionError;

#[derive(Accounts)]
#[instruction(deposit_id: [u8; 16])]
pub struct DepositSettleInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [BankingSettlementConfig::SEED],
        bump = banking_settlement_config.bump,
    )]
    pub banking_settlement_config: Account<'info, BankingSettlementConfig>,
    #[account(
        constraint = operator.key() == banking_settlement_config.operator
            @ DepositSettleInstructionError::UnauthorizedOperator,
    )]
    pub operator: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + DepositSettlementReceipt::LEN,
        seeds = [DepositSettlementReceipt::SEED, deposit_id.as_ref()],
        bump,
    )]
    pub deposit_settlement_receipt: Account<'info, DepositSettlementReceipt>,
    /// CHECK: PDA validated by seeds and used as the program-controlled pool authority.
    #[account(seeds = [PoolAuthority::SEED], bump)]
    pub pool_authority: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = pool_authority,
        associated_token::token_program = token_program,
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_account_owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = token_account_owner,
        associated_token::token_program = token_program,
    )]
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
