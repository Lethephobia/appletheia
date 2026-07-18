use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while completing a withdrawal.
#[derive(Debug, Error)]
pub enum WithdrawalCompleteCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),
}

impl Retryability for WithdrawalCompleteCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::Withdrawal(_) => false,
        }
    }
}
