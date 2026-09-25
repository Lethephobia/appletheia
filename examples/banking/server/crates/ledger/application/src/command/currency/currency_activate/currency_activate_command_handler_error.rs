use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyActivateCommandHandlerError {
    #[error("currency repository failed")]
    Repository(#[from] RepositoryError<Currency>),
    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),
}

impl Retryability for CurrencyActivateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Currency(_) => false,
        }
    }
}
