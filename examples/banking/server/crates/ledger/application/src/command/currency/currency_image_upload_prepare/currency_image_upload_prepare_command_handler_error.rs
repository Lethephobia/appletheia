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

    #[error("currency was not found")]
    CurrencyNotFound,

    #[error("removed currencies cannot prepare image uploads")]
    CurrencyRemoved,

    #[error("image content length exceeds the configured maximum")]
    ContentLengthTooLarge,

    #[error("image content type is not allowed")]
    ContentTypeNotAllowed,
}
