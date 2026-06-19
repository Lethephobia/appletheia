use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use thiserror::Error;

use crate::mint::{MintAccountSeedError, MintSupplySynchronizerError};

/// Represents errors returned while synchronizing on-chain mint supply.
#[derive(Debug, Error)]
pub enum CurrencyMintSupplySyncCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("mint account seed is invalid")]
    MintAccountSeed(#[from] MintAccountSeedError),

    #[error("mint supply synchronizer failed")]
    MintSupplySynchronizer(#[from] MintSupplySynchronizerError),

    #[error("currency mint account has not been recorded yet")]
    MintAccountNotRecorded,
}
