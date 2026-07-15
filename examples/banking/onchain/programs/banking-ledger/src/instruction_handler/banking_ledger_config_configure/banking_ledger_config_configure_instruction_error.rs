use anchor_lang::prelude::*;

#[error_code]
pub enum BankingLedgerConfigConfigureInstructionError {
    #[msg("program account does not match the banking ledger program")]
    ProgramAccountMismatch,
    #[msg("program data account does not match the banking ledger program")]
    ProgramDataAccountMismatch,
    #[msg("upgrade authority is not authorized")]
    UnauthorizedUpgradeAuthority,
    #[msg("program has no upgrade authority")]
    ProgramUpgradeAuthorityMissing,
}
