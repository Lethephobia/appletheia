use super::BankingLedgerConfigConfigurerError;

/// Configures the on-chain banking ledger config.
#[allow(async_fn_in_trait)]
pub trait BankingLedgerConfigConfigurer: Send + Sync {
    async fn configure(&self) -> Result<(), BankingLedgerConfigConfigurerError>;
}
