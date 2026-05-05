use std::error::Error;

use thiserror::Error;

/// Error returned while loading transfer recipient list read models.
#[derive(Debug, Error)]
pub enum TransferRecipientListItemReaderError {
    #[error("transfer recipient list item reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
