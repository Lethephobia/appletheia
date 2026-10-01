use appletheia::application::Retryability;
use appletheia::application::repository::{ReferenceIndexLookupError, RepositoryError};
use banking_ledger_domain::owned_account_closure::{OwnedAccountClosure, OwnedAccountClosureError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OwnedAccountClosureScanCommandHandlerError {
    #[error("owned account closure repository failed")]
    OwnedAccountClosureRepository(#[from] RepositoryError<OwnedAccountClosure>),

    #[error("reference index lookup failed")]
    ReferenceIndexLookup(#[from] ReferenceIndexLookupError),

    #[error("owned account closure aggregate failed")]
    OwnedAccountClosure(#[from] OwnedAccountClosureError),

    #[error("owned account closure is already completed")]
    AlreadyCompleted,

    #[error("account scan is already complete")]
    ScanAlreadyCompleted,
}

impl Retryability for OwnedAccountClosureScanCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OwnedAccountClosureRepository(error) => error.is_retryable(),
            Self::ReferenceIndexLookup(error) => error.is_retryable(),
            Self::OwnedAccountClosure(_) | Self::AlreadyCompleted | Self::ScanAlreadyCompleted => {
                false
            }
        }
    }
}
