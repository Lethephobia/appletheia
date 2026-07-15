use std::error::Error;

use thiserror::Error;

/// Error returned while writing user-private read models.
#[derive(Debug, Error)]
pub enum UserPrivateInfoWriterError {
    #[error("user private info writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
