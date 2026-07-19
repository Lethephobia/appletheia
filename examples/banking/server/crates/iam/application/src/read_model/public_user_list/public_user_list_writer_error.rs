use std::error::Error;

use thiserror::Error;

/// Error returned while writing public user list read models.
#[derive(Debug, Error)]
pub enum PublicUserListWriterError {
    #[error("public user list writer persistence failed")]
    Persistence(#[source] Box<dyn Error + Send + Sync>),
}
