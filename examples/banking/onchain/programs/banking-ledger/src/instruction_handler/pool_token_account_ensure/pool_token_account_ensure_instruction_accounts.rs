use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint as TokenMint, TokenAccount},
};

use crate::account::{BankingLedgerConfig, Mint, MintState, ProgramAuthority};
use crate::instruction_handler::PoolTokenAccountEnsureInstructionError;

#[derive(Accounts)]
#[instruction(mint_id: [u8; 16])]
pub struct PoolTokenAccountEnsureInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [BankingLedgerConfig::SEED],
        bump = banking_ledger_config.bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        constraint = operator.key() == banking_ledger_config.operator
            @ PoolTokenAccountEnsureInstructionError::UnauthorizedOperator,
    )]
    pub operator: Signer<'info>,
    #[account(
        seeds = [MintState::SEED, mint_id.as_ref()],
        bump = mint_state.bump,
        constraint = mint_state.is_initialized()
            @ PoolTokenAccountEnsureInstructionError::MintStateNotInitialized,
        constraint = mint_state.version == MintState::VERSION
            && mint_state.mint_id == mint_id
            @ PoolTokenAccountEnsureInstructionError::MintStateConflict,
    )]
    pub mint_state: Account<'info, MintState>,
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
    )]
    pub mint: InterfaceAccount<'info, TokenMint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = program_authority,
        associated_token::token_program = token_program,
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
