use crate::read_model::OwnedAccountListReaderError;
use thiserror::Error;

/// Error returned while handling account list queries.
#[derive(Debug, Error)]
pub enum OwnedAccountListQueryHandlerError {
    #[error("account list reader failed")]
    Reader(#[from] OwnedAccountListReaderError),
}
