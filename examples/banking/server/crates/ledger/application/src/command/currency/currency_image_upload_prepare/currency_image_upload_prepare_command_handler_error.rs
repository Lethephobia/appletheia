use appletheia::application::Retryability;

use appletheia::application::object_storage::{ObjectNameError, ObjectUploadSignerError};
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while preparing a currency-image upload.
#[derive(Debug, Error)]
pub enum CurrencyImageUploadPrepareCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("object storage object name generation failed")]
    ObjectName(#[from] ObjectNameError),

    #[error("object upload signer failed")]
    ObjectUploadSigner(#[from] ObjectUploadSignerError),
}

impl Retryability for CurrencyImageUploadPrepareCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::ObjectName(_) => false,
            Self::ObjectUploadSigner(error) => error.is_retryable(),
        }
    }
}
