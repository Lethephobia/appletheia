use std::error::Error;

use thiserror::Error;

/// Error returned while writing owned account list read models.
#[derive(Debug, Error)]
pub enum OwnedAccountListWriterError {
    #[error("owned account list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
