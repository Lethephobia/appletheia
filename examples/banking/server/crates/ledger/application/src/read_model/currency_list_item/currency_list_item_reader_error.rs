use std::error::Error;

use thiserror::Error;

/// Error returned while loading currency list read models.
#[derive(Debug, Error)]
pub enum CurrencyListItemReaderError {
    #[error("currency list item reader persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
