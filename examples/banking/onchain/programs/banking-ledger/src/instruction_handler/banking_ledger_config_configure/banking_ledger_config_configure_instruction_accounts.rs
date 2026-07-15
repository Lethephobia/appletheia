use anchor_lang::prelude::*;

use crate::account::BankingLedgerConfig;
use crate::instruction_handler::BankingLedgerConfigConfigureInstructionError;
use crate::ID;

#[derive(Accounts)]
pub struct BankingLedgerConfigConfigureInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Stored as the configured banking ledger operator.
    pub operator: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + BankingLedgerConfig::LEN,
        seeds = [BankingLedgerConfig::SEED],
        bump,
    )]
    pub banking_ledger_config: Account<'info, BankingLedgerConfig>,
    #[account(
        address = ID @ BankingLedgerConfigConfigureInstructionError::ProgramAccountMismatch,
        constraint = program.programdata_address()? == Some(program_data.key())
            @ BankingLedgerConfigConfigureInstructionError::ProgramDataAccountMismatch,
    )]
    pub program: Program<'info>,
    pub program_data: Account<'info, ProgramData>,
    #[account(
        constraint = program_data.upgrade_authority_address.is_some()
            @ BankingLedgerConfigConfigureInstructionError::ProgramUpgradeAuthorityMissing,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
            @ BankingLedgerConfigConfigureInstructionError::UnauthorizedUpgradeAuthority,
    )]
    pub upgrade_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
