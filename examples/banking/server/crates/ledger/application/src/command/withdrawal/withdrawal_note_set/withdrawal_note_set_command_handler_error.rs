use appletheia::application::Retryability;
use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WithdrawalNoteSetCommandHandlerError {
    #[error("withdrawal repository failed")]
    WithdrawalRepository(#[from] RepositoryError<Withdrawal>),

    #[error("withdrawal aggregate failed")]
    Withdrawal(#[from] WithdrawalError),
}

impl Retryability for WithdrawalNoteSetCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::WithdrawalRepository(error) => error.is_retryable(),
            Self::Withdrawal(_) => false,
        }
    }
}
