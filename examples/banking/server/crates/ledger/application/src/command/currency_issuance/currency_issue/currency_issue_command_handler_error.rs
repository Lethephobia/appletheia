use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceError};
use thiserror::Error;

/// Represents errors returned while starting a currency issuance.
#[derive(Debug, Error)]
pub enum CurrencyIssueCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency issuance repository failed")]
    CurrencyIssuanceRepository(#[from] RepositoryError<CurrencyIssuance>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency issuance aggregate failed")]
    CurrencyIssuance(#[from] CurrencyIssuanceError),

    #[error("destination account was not found")]
    DestinationAccountNotFound,

    #[error("currency was not found")]
    CurrencyNotFound,

    #[error("destination account currency does not match")]
    CurrencyMismatch,

    #[error("currency issuance id was missing")]
    MissingCurrencyIssuanceId,
}
