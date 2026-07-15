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
