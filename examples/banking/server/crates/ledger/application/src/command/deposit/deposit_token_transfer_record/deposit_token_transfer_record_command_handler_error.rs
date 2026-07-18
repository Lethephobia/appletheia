use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::deposit::{Deposit, DepositError};
use thiserror::Error;

use crate::mint::TokenDepositVerifierError;

/// Represents errors returned while recording a deposit token transfer.
#[derive(Debug, Error)]
pub enum DepositTokenTransferRecordCommandHandlerError {
    #[error("deposit repository failed")]
    DepositRepository(#[from] RepositoryError<Deposit>),

    #[error("deposit aggregate failed")]
    Deposit(#[from] DepositError),

    #[error("token deposit verification failed")]
    TokenDepositVerifier(#[from] TokenDepositVerifierError),
}

impl Retryability for DepositTokenTransferRecordCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::DepositRepository(error) => error.is_retryable(),
            Self::Deposit(_) => false,
            Self::TokenDepositVerifier(error) => error.is_retryable(),
        }
    }
}
