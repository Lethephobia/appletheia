use appletheia::application::Retryability;

use appletheia::application::repository::RepositoryError;
use banking_ledger_domain::owned_account_closure::{OwnedAccountClosure, OwnedAccountClosureError};
use thiserror::Error;

/// Represents errors returned while failing an owned account closure.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureFailCommandHandlerError {
    #[error("owned account closure repository failed")]
    OwnedAccountClosureRepository(#[from] RepositoryError<OwnedAccountClosure>),

    #[error("owned account closure aggregate failed")]
    OwnedAccountClosure(#[from] OwnedAccountClosureError),
}

impl Retryability for OwnedAccountClosureFailCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OwnedAccountClosureRepository(error) => error.is_retryable(),
            Self::OwnedAccountClosure(_) => false,
        }
    }
}
