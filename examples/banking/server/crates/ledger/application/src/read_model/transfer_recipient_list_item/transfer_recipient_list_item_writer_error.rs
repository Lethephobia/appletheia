use std::error::Error;

use thiserror::Error;

/// Error returned while writing transfer recipient list item read models.
#[derive(Debug, Error)]
pub enum TransferRecipientListItemWriterError {
    #[error("transfer recipient list item writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
