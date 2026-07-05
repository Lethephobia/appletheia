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
