use appletheia::application::repository::{ReferenceIndexLookupError, RepositoryError};
use banking_ledger_domain::owned_account_closure::{OwnedAccountClosure, OwnedAccountClosureError};
use thiserror::Error;

/// Represents errors returned while loading an owned account closure page.
#[derive(Debug, Error)]
pub enum OwnedAccountClosurePageLoadCommandHandlerError {
    #[error("owned account closure repository failed")]
    OwnedAccountClosureRepository(#[from] RepositoryError<OwnedAccountClosure>),

    #[error("reference index lookup failed")]
    ReferenceIndexLookup(#[from] ReferenceIndexLookupError),

    #[error("owned account closure aggregate failed")]
    OwnedAccountClosure(#[from] OwnedAccountClosureError),

    #[error("owned account closure was not found")]
    OwnedAccountClosureNotFound,

    #[error("page size must not be zero")]
    ZeroPageSize,
}
