use anchor_lang::prelude::*;

use crate::account::BankingSettlementConfig;
use crate::instruction_handler::BankingSettlementConfigConfigureInstructionError;
use crate::ID;

#[derive(Accounts)]
pub struct BankingSettlementConfigConfigureInstructionAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Stored as the configured banking settlement operator.
    pub operator: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + BankingSettlementConfig::LEN,
        seeds = [BankingSettlementConfig::SEED],
        bump,
    )]
    pub banking_settlement_config: Account<'info, BankingSettlementConfig>,
    #[account(
        address = ID @ BankingSettlementConfigConfigureInstructionError::ProgramAccountMismatch,
        constraint = program.programdata_address()? == Some(program_data.key())
            @ BankingSettlementConfigConfigureInstructionError::ProgramDataAccountMismatch,
    )]
    pub program: Program<'info>,
    pub program_data: Account<'info, ProgramData>,
    #[account(
        constraint = program_data.upgrade_authority_address.is_some()
            @ BankingSettlementConfigConfigureInstructionError::ProgramUpgradeAuthorityMissing,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
            @ BankingSettlementConfigConfigureInstructionError::UnauthorizedUpgradeAuthority,
    )]
    pub upgrade_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
