use crate::read_model::WalletBookmarkListReaderError;
use thiserror::Error;

/// Error returned while handling wallet bookmark list queries.
#[derive(Debug, Error)]
pub enum WalletBookmarkListQueryHandlerError {
    #[error("wallet bookmark list reader failed")]
    Reader(#[from] WalletBookmarkListReaderError),
}
