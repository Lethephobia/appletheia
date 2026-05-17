use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while changing a currency symbol.
#[derive(Debug, Error)]
pub enum CurrencySymbolChangeCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("currency was not found")]
    CurrencyNotFound,
}
