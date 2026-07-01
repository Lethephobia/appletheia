use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;

use crate::account::{BankingLedgerConfig, MintMetadata, MintState, ProgramAuthority};
use crate::instruction_handler::MintMetadataUpdateInstructionError;

#[derive(Accounts)]
#[instruction(mint_id: [u8; 16])]
pub struct MintMetadataUpdateInstructionAccounts<'info> {
    #[account(
        seeds = [BankingLedgerConfig::SEED],
        bump = banking_ledger_config.bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        constraint = operator.key() == banking_ledger_config.operator
            @ MintMetadataUpdateInstructionError::UnauthorizedOperator,
    )]
    pub operator: Signer<'info>,
    #[account(
        seeds = [MintState::SEED, mint_id.as_ref()],
        bump = mint_state.bump,
        constraint = mint_state.is_initialized()
            @ MintMetadataUpdateInstructionError::MintStateNotInitialized,
        constraint = mint_state.version == MintState::VERSION
            && mint_state.mint_id == mint_id
            @ MintMetadataUpdateInstructionError::MintStateConflict,
    )]
    pub mint_state: Account<'info, MintState>,
    /// CHECK: PDA validated by seeds and used as the program-controlled authority.
    #[account(
        seeds = [ProgramAuthority::SEED],
        bump = mint_state.program_authority_bump,
    )]
    pub program_authority: UncheckedAccount<'info>,
    /// CHECK: PDA validated by seeds and updated as Token-2022 metadata by this instruction.
    #[account(
        mut,
        owner = token_program.key() @ MintMetadataUpdateInstructionError::MintMetadataAccountConflict,
        seeds = [MintMetadata::SEED, mint_id.as_ref()],
        bump = mint_state.mint_metadata_bump,
    )]
    pub mint_metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token2022>,
}
