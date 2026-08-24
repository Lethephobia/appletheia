use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use appletheia::domain::UniqueValueError;
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarHandleChangeCommandHandlerError {
    #[error("currency registrar repository failed")]
    Repository(#[from] RepositoryError<CurrencyRegistrar>),
    #[error("currency registrar aggregate failed")]
    Aggregate(#[from] CurrencyRegistrarError),
    #[error("currency registrar unique value is invalid")]
    UniqueValue(#[from] UniqueValueError),
}

impl Retryability for CurrencyRegistrarHandleChangeCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Repository(error) => error.is_retryable(),
            Self::Aggregate(_) | Self::UniqueValue(_) => false,
        }
    }
}
