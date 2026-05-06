use std::error::Error;

use thiserror::Error;

/// Error returned while loading account list read models.
#[derive(Debug, Error)]
pub enum OwnedAccountListItemReaderError {
    #[error("owned account list item reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
