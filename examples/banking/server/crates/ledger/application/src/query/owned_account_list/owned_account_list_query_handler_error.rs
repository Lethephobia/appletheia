use super::OwnedAccountListStoreError;
use thiserror::Error;

/// Error returned while handling account list queries.
#[derive(Debug, Error)]
pub enum OwnedAccountListQueryHandlerError {
    #[error("account list store failed")]
    Store(#[from] OwnedAccountListStoreError),
}
