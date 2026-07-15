use std::error::Error;

use thiserror::Error;

/// Error returned while writing currency list read models.
#[derive(Debug, Error)]
pub enum CurrencyListWriterError {
    #[error("currency list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
