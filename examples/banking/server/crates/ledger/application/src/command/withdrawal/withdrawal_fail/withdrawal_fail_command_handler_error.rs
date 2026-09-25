use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

/// Represents errors returned while failing a withdrawal.
#[derive(Debug, Error)]
pub enum WithdrawalFailCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),
}

impl Retryability for WithdrawalFailCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::Withdrawal(_) => false,
        }
    }
}
