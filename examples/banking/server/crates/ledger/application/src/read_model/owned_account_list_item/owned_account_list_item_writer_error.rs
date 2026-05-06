use std::error::Error;

use thiserror::Error;

/// Error returned while writing owned account list item read models.
#[derive(Debug, Error)]
pub enum OwnedAccountListItemWriterError {
    #[error("owned account list item writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
