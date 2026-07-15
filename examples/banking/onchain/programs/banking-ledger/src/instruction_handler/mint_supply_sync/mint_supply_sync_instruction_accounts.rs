use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint as TokenMint, TokenAccount},
};

use crate::account::{BankingLedgerConfig, Mint, MintState, ProgramAuthority};
use crate::instruction_handler::MintSupplySyncInstructionError;

#[derive(Accounts)]
#[instruction(mint_id: [u8; 16])]
pub struct MintSupplySyncInstructionAccounts<'info> {
    #[account(
        seeds = [BankingLedgerConfig::SEED],
        bump = banking_ledger_config.bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        constraint = operator.key() == banking_ledger_config.operator
            @ MintSupplySyncInstructionError::UnauthorizedOperator,
    )]
    pub operator: Signer<'info>,
    #[account(
        seeds = [MintState::SEED, mint_id.as_ref()],
        bump = mint_state.bump,
        constraint = mint_state.is_initialized()
            @ MintSupplySyncInstructionError::MintStateNotInitialized,
        constraint = mint_state.version == MintState::VERSION
            @ MintSupplySyncInstructionError::MintStateConflict,
    )]
    pub mint_state: Account<'info, MintState>,
    /// CHECK: PDA validated by seeds and used as the program-controlled authority.
    #[account(
        seeds = [ProgramAuthority::SEED],
        bump = mint_state.program_authority_bump,
    )]
    pub program_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [Mint::SEED, mint_id.as_ref()],
        bump = mint_state.mint_bump,
        mint::token_program = token_program,
        constraint = mint.mint_authority == COption::Some(program_authority.key())
            @ MintSupplySyncInstructionError::MintAccountConflict,
    )]
    pub mint: InterfaceAccount<'info, TokenMint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = program_authority,
        associated_token::token_program = token_program,
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
