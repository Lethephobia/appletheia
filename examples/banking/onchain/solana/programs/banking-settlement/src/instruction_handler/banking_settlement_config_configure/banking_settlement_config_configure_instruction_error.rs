use anchor_lang::prelude::*;

#[error_code]
pub enum BankingSettlementConfigConfigureInstructionError {
    #[msg("program account does not match the banking settlement program")]
    ProgramAccountMismatch,
    #[msg("program data account does not match the banking settlement program")]
    ProgramDataAccountMismatch,
    #[msg("upgrade authority is not authorized")]
    UnauthorizedUpgradeAuthority,
    #[msg("program has no upgrade authority")]
    ProgramUpgradeAuthorityMissing,
}
