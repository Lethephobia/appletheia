use appletheia::application::Retryability;

use crate::mint::MintMetadataUpdaterError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

/// Represents errors returned while synchronizing currency mint metadata.
#[derive(Debug, Error)]
pub enum MintMetadataSyncCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint metadata updater failed")]
    MintMetadataUpdater(#[from] MintMetadataUpdaterError),
}

impl Retryability for MintMetadataSyncCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::MintMetadataUpdater(error) => error.is_retryable(),
        }
    }
}
