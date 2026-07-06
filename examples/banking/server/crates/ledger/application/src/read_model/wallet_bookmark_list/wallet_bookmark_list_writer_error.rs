use std::error::Error;

use thiserror::Error;

/// Error returned while writing wallet bookmark list read models.
#[derive(Debug, Error)]
pub enum WalletBookmarkListWriterError {
    #[error("wallet bookmark list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
