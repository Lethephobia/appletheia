use std::error::Error;

use thiserror::Error;

/// Error returned while writing owned account transaction list read models.
#[derive(Debug, Error)]
pub enum OwnedAccountTransactionListItemWriterError {
    #[error("owned account transaction list item writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
