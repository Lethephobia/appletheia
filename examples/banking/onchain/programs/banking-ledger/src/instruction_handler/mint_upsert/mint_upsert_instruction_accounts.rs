use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::Mint as TokenMint;

use crate::account::{BankingLedgerConfig, Mint, MintMetadata, MintState, ProgramAuthority};
use crate::instruction_handler::MintUpsertInstructionError;

#[derive(Accounts)]
#[instruction(mint_id: [u8; 16], decimals: u8)]
pub struct MintUpsertInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [BankingLedgerConfig::SEED],
        bump = banking_ledger_config.bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        constraint = operator.key() == banking_ledger_config.operator
            @ MintUpsertInstructionError::UnauthorizedOperator,
    )]
    pub operator: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + MintState::LEN,
        seeds = [MintState::SEED, mint_id.as_ref()],
        bump,
    )]
    pub mint_state: Account<'info, MintState>,
    /// CHECK: PDA validated by seeds and used as the program-controlled authority.
    #[account(
        seeds = [ProgramAuthority::SEED],
        bump,
    )]
    pub program_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [Mint::SEED, mint_id.as_ref()],
        bump,
        mint::decimals = decimals,
        mint::authority = program_authority,
        mint::freeze_authority = program_authority,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = program_authority,
        extensions::metadata_pointer::metadata_address = mint_metadata,
        constraint = mint.decimals == decimals @ MintUpsertInstructionError::MintAccountConflict,
        constraint = mint.mint_authority == COption::Some(program_authority.key())
            @ MintUpsertInstructionError::MintAccountConflict,
        constraint = mint.freeze_authority == COption::Some(program_authority.key())
            @ MintUpsertInstructionError::MintAccountConflict,
    )]
    pub mint: InterfaceAccount<'info, TokenMint>,
    /// CHECK: PDA validated by seeds and initialized as Token-2022 metadata by this instruction.
    #[account(
        init_if_needed,
        payer = payer,
        space = MintMetadata::SPACE,
        owner = token_program.key(),
        seeds = [MintMetadata::SEED, mint_id.as_ref()],
        bump,
    )]
    pub mint_metadata: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}
