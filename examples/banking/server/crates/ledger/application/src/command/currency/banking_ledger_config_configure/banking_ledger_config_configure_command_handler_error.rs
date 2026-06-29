use thiserror::Error;

use crate::banking_ledger::BankingLedgerConfigConfigurerError;

/// Represents errors returned while configuring the on-chain banking ledger config.
#[derive(Debug, Error)]
pub enum BankingLedgerConfigConfigureCommandHandlerError {
    #[error("banking ledger config configurer failed")]
    BankingLedgerConfigConfigurer(#[from] BankingLedgerConfigConfigurerError),
}
