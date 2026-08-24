use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyDefineCommandHandlerError {
    #[error("currency repository failed")]
    Repository(#[from] RepositoryError<Currency>),
    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),
    #[error("currency unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
    #[error("currency code is already defined")]
    DuplicateCode,
}

impl Retryability for CurrencyDefineCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Currency(_) | Self::UniqueValue(_) | Self::DuplicateCode => false,
        }
    }
}
