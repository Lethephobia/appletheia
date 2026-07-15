use std::error::Error;

use thiserror::Error;

/// Error returned while loading wallet bookmark list read models.
#[derive(Debug, Error)]
pub enum WalletBookmarkListReaderError {
    #[error("wallet bookmark list reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
