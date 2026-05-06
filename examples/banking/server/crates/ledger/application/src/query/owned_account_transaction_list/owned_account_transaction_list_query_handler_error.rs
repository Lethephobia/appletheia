use thiserror::Error;

use crate::read_model::OwnedAccountTransactionListItemReaderError;

/// Error returned while handling owned account transaction list queries.
#[derive(Debug, Error)]
pub enum OwnedAccountTransactionListQueryHandlerError {
    #[error(transparent)]
    Reader(#[from] OwnedAccountTransactionListItemReaderError),
}
