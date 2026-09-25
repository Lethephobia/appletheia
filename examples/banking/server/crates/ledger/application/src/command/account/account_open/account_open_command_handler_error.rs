use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::account::{Account, AccountError};
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while opening an account.
#[derive(Debug, Error)]
pub enum AccountOpenCommandHandlerError {
    #[error("account repository failed")]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error("account aggregate failed")]
    Account(#[from] AccountError),

    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("currency is inactive")]
    CurrencyInactive,
}

impl Retryability for AccountOpenCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) | Self::CurrencyInactive => false,
        }
    }
}
