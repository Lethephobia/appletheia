use thiserror::Error;

/// Represents errors returned while configuring the on-chain banking ledger config.
#[derive(Debug, Error)]
pub enum BankingLedgerConfigConfigurerError {
    #[error("banking ledger config configurer backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
