use thiserror::Error;

use crate::read_model::TransferRecipientListItemReaderError;

/// Error returned while handling transfer recipient list queries.
#[derive(Debug, Error)]
pub enum TransferRecipientListQueryHandlerError {
    #[error("transfer recipient list item reader failed")]
    Reader(#[from] TransferRecipientListItemReaderError),
}
