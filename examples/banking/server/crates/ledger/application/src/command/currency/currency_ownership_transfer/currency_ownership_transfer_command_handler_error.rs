use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while transferring currency ownership.
#[derive(Debug, Error)]
pub enum CurrencyOwnershipTransferCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),
}

impl Retryability for CurrencyOwnershipTransferCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
        }
    }
}
