use std::error::Error;

use thiserror::Error;

/// Error returned while reading owned account transaction list read models.
#[derive(Debug, Error)]
pub enum OwnedAccountTransactionListItemReaderError {
    #[error("owned account transaction list item reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
