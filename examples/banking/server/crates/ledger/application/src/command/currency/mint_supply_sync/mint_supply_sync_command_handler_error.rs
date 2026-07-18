use appletheia::application::Retryability;

use crate::mint::MintSupplySynchronizerError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while synchronizing on-chain mint supply.
#[derive(Debug, Error)]
pub enum MintSupplySyncCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint supply synchronizer failed")]
    MintSupplySynchronizer(#[from] MintSupplySynchronizerError),
}

impl Retryability for MintSupplySyncCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::MintSupplySynchronizer(error) => error.is_retryable(),
        }
    }
}
