use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarDescriptionChangeCommandHandlerError {
    #[error("currency registrar repository failed")]
    Repository(#[from] RepositoryError<CurrencyRegistrar>),
    #[error("currency registrar aggregate failed")]
    Aggregate(#[from] CurrencyRegistrarError),
}

impl Retryability for CurrencyRegistrarDescriptionChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Aggregate(_) => false,
        }
    }
}
