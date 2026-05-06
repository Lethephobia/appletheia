use std::error::Error;

use thiserror::Error;

/// Error returned while writing currency list item read models.
#[derive(Debug, Error)]
pub enum CurrencyListItemWriterError {
    #[error("currency list item writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
