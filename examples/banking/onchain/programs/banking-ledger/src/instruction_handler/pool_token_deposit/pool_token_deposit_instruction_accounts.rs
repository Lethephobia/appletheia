use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint as TokenMint, TokenAccount},
};

use crate::account::{
    BankingLedgerConfig, Mint, MintState, PoolTokenDepositMarker, ProgramAuthority,
};
use crate::instruction_handler::PoolTokenDepositInstructionError;

#[derive(Accounts)]
#[instruction(idempotency_key: [u8; 16], mint_id: [u8; 16])]
pub struct PoolTokenDepositInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [BankingLedgerConfig::SEED],
        bump = banking_ledger_config.bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        constraint = operator.key() == banking_ledger_config.operator
            @ PoolTokenDepositInstructionError::UnauthorizedOperator,
    )]
    pub operator: UncheckedAccount<'info>,
    #[account(
        seeds = [MintState::SEED, mint_id.as_ref()],
        bump = mint_state.bump,
        constraint = mint_state.is_initialized()
            @ PoolTokenDepositInstructionError::MintStateNotInitialized,
        constraint = mint_state.version == MintState::VERSION
            @ PoolTokenDepositInstructionError::MintStateConflict,
    )]
    pub mint_state: Account<'info, MintState>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + PoolTokenDepositMarker::LEN,
        seeds = [PoolTokenDepositMarker::SEED, idempotency_key.as_ref()],
        bump,
    )]
    pub pool_token_deposit_marker: Account<'info, PoolTokenDepositMarker>,
    /// CHECK: PDA validated by seeds and used as the program-controlled authority.
    #[account(
        seeds = [ProgramAuthority::SEED],
        bump = mint_state.program_authority_bump,
    )]
    pub program_authority: UncheckedAccount<'info>,
    #[account(
        seeds = [Mint::SEED, mint_id.as_ref()],
        bump = mint_state.mint_bump,
        mint::token_program = token_program,
        constraint = mint.mint_authority == COption::Some(program_authority.key())
            @ PoolTokenDepositInstructionError::MintAccountConflict,
    )]
    pub mint: InterfaceAccount<'info, TokenMint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = program_authority,
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
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
