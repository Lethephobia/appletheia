use appletheia::application::Retryability;

use crate::mint::PoolTokenTransferExecutorError;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::currency::{Currency, CurrencyError};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while executing a withdrawal pool token transfer.
#[derive(Debug, Error)]
pub enum WithdrawalTokenTransferCommandHandlerError {
    #[error("currency repository failed")]
    CurrencyRepository(#[from] RepositoryError<Currency>),

    #[error("currency aggregate failed")]
    Currency(#[from] CurrencyError),

    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),

    #[error("pool token transfer executor failed")]
    PoolTokenTransferExecutor(#[from] PoolTokenTransferExecutorError),
}

impl Retryability for WithdrawalTokenTransferCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::CurrencyRepository(error) => error.is_retryable(),
            Self::Currency(_) => false,
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::Withdrawal(_) => false,
            Self::PoolTokenTransferExecutor(error) => error.is_retryable(),
        }
    }
}
